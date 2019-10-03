extern crate futures;
extern crate tokio;

use std::ops::Add;
use std::time::{Duration, Instant};

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::{Future, Stream};

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    let grant_lease = client
        .lease()
        .grant(GrantRequest::new(10))
        .map_err(|_| ())
        .and_then(move |resp| {
            let lease_id = resp.id();
            let lease_client = client.lease();
            let keep_alive = lease_client
                .keep_alive(KeepAliveRequest::new(lease_id))
                .then(|res| match res {
                    Ok(resp) => {
                        if resp.ttl() == 0 {
                            println!("lease expired");
                            Err(())
                        } else {
                            println!("keeping alive: {:?}", resp);
                            Ok(())
                        }
                    }
                    Err(err) => {
                        println!("failed to keep alive: {:?}", err);
                        Err(())
                    }
                })
                .for_each(|_| Ok(()));

            tokio::spawn(keep_alive);

            // print key value every 1 second
            let print_kvs = {
                let kv_client = client.kv();
                tokio::timer::Interval::new_interval(Duration::from_secs(1))
                    .map_err(|_| ())
                    .for_each(move |_| {
                        let print_kv = kv_client
                            .get(GetRequest::key("foo"))
                            .map(|resp| {
                                println!("key values: {:?}", resp);
                            })
                            .map_err(|_| ());
                        tokio::spawn(print_kv);

                        Ok(())
                    })
            };

            tokio::spawn(print_kvs);

            // put key value
            let put_kv = client
                .kv()
                .put(PutRequest::new("foo", "bar").with_lease(lease_id))
                .map(|_| ())
                .map_err(|e| {
                    println!("failed to put key value: {:?}", e);
                    ()
                });

            tokio::spawn(put_kv);

            // revoke lease after 30 seconds
            let revoke_lease =
                tokio::timer::Delay::new(Instant::now().add(Duration::from_secs(30)))
                    .map_err(|_| ())
                    .and_then(move |_| {
                        client
                            .lease()
                            .revoke(RevokeRequest::new(lease_id))
                            .map(|_| ())
                            .map_err(|e| {
                                println!("failed to revoke lease: {:?}", e);
                                ()
                            })
                    });

            revoke_lease
        });

    tokio::run(grant_lease);
}
