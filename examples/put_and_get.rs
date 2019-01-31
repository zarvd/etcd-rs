extern crate futures;
extern crate tokio;

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::{lazy, Future, Stream};

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    tokio::run(
        client
            .kv()
            .get(GetRequest::prefix("/foo/"))
            .map(|resp| {
                println!("header: {:?}", resp.header());
                println!("count: {}", resp.count());
                for kv in resp.kvs() {
                    println!("{:?}", kv);
                }
            })
            .map_err(|_| ()),
    );

    // tokio::run(
    //     client
    //         .kv()
    //         .put(PutRequest::new("foo", "bar"))
    //         .map_err(|_| ())
    //         .then(move |_| {
    //             client
    //                 .kv()
    //                 .get(GetRequest::key("foo"))
    //                 .map(|kvs| {
    //                     for (k, v) in kvs {
    //                         println!("k: {}, v: {}", k, v);
    //                     }
    //                 })
    //                 .map_err(|_| ())
    //                 .then(move |_| {
    //                     client
    //                         .kv()
    //                         .delete(DeleteRequest::key("foo"))
    //                         .map_err(|_| ())
    //                 })
    //         }),
    // );
}
