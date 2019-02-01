extern crate futures;
extern crate tokio;

use etcd_rs::prelude::*;
use etcd_rs::Client;

use futures::{Future, Stream};

fn main() {
    let client = Client::builder().add_endpoint("127.0.0.1:2379").build();

    tokio::run(
        client
            .watch()
            .watch(WatchRequest::prefix("/"))
            .for_each(|resp| {
                println!("watch event: {:?}", resp);
                Ok(())
            })
            .map_err(|e| {
                println!("failed to recv watch events: {:?}", e);
            }),
    );
}
