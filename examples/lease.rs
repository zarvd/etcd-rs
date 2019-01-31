extern crate futures;
extern crate grpcio;
extern crate tokio;

use std::ops::Add;
use std::time::{Duration, Instant};

use etcd_rs::prelude::*;
use etcd_rs::{Client, Error};

use futures::{Future, Sink, Stream};

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    let grant_lease = client
        .lease()
        .grant(GrantRequest::new(10))
        .map_err(|_| ())
        .and_then(move |resp| {
            let lease_id = resp.id();

            let keep_alive = {
                // keep alive
                let r = KeepAliveRequest::new(lease_id);
                let (req, reply) = client.lease().keep_alive();

                // send keep alive request
                let keep_alive_request = req
                    .send_all(
                        tokio::timer::Interval::new_interval(Duration::from_millis(10))
                            .and_then(move |_| Ok((r.clone().into(), Default::default())))
                            .map_err(|_| grpcio::Error::GoogleAuthenticationFailed),
                    )
                    .map_err(|e| println!("failed to send keep alive request: {:?}", e));

                // .forward(req)
                // .map_err(|e| println!("failed to send keep alive request: {:?}", e));

                // recv keep alive reply
                let keep_alive_reply = reply
                    .for_each(|resp| {
                        println!("keeping alive: {:?}", resp);
                        Ok(())
                    })
                    .map_err(|e| println!("keep alive failed: {:?}", e));

                keep_alive_request.join(keep_alive_reply).map(|_| ())
            };

            keep_alive
        });

    tokio::run(grant_lease);
}
