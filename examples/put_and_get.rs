extern crate futures;
extern crate tokio;

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::Future;

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    let op = client
        .kv()
        .put(PutRequest::new("foo", "bar")) // put new key-value
        .map_err(|e| println!("failed to put key into etcd: {:?}", e))
        .and_then(move |_| {
            client
                .kv()
                .get(GetRequest::key("foo")) // get key-value
                .map_err(|e| println!("failed to fetch key from etcd: {:?}", e))
                .and_then(move |resp| {
                    for kv in resp.kvs() {
                        println!("k: {}, v: {}", kv.key().unwrap(), kv.value().unwrap());
                    }
                    client
                        .kv()
                        .delete(DeleteRequest::key("foo")) // delete the key
                        .map(|_| ())
                        .map_err(|e| println!("failed to delete key from etcd: {:?}", e))
                })
        });
    tokio::run(op);
}
