extern crate futures;
extern crate tokio;

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::Future;

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    let op = client
        .kv()
        .txn(
            TxnRequest::new()
                .when_value("foo", TxnCmp::Equal, "bar")
                .and_then(PutRequest::new("result", "good"))
                .or_else(PutRequest::new("result", "bad")),
        )
        .map_err(|e| println!("exec transaction encouter error: {:?}", e))
        .and_then(move |resp| {
            if resp.is_succeeded() {
                println!("transaction exec succeeded");
            } else {
                println!("transaction exec failed");
            }

            for result in resp.results() {
                println!("transaction result: {:?}", result);
            }

            client
                .kv()
                .get(GetRequest::key("result"))
                .map(|resp| {
                    let kv = &resp.kvs()[0];
                    // result: good or result: bad
                    println!("key: {}, value: {}", kv.key().unwrap(), kv.value().unwrap());
                })
                .map_err(|e| println!("failed to fetch key-value 'result': {:?}", e))
        });
    tokio::run(op);
}
