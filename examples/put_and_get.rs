extern crate futures;
extern crate tokio;

use etcd_rs::Client;

use futures::{lazy, Future};

fn main() {
    tokio::run(lazy(|| {
        let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

        tokio::spawn(client.cluster().member_list().map(|ids| {
            for id in ids {
                println!("id: {}", id);
            }
        }));

        tokio::spawn(client.kv().get("foo").map(|kvs| {
            for (k, v) in kvs {
                println!("k: {}, v: {}", k, v);
            }
        }));

        Ok(())
    }));
}
