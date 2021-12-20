use rand::Rng;

use etcd_rs::*;

mod support;
use crate::support::Context;

async fn put_and_get(cli: &Client, retry: usize) {
    use std::time::Duration;
    for i in 0..retry + 1 {
        tokio::time::sleep(Duration::from_secs(3));

        let v = rand::thread_rng().gen::<u64>().to_string();
        let r = cli.kv().put(PutRequest::new("foo", v.clone())).await;
        if r.is_err() {
            continue;
        }

        let resp = cli
            .kv()
            .range(RangeRequest::new(KeyRange::key("foo")))
            .await;
        if resp.is_err() {
            continue;
        }

        let mut resp = resp.unwrap();

        assert_eq!(1, resp.count());
        let kvs = resp.take_kvs();
        assert_eq!(&v, kvs[0].value_str());

        break;
    }
}

async fn expect_timeout(cli: &Client) {
    let res = cli.kv().put(PutRequest::new("foo", "bar")).await; // FIXME check specified error

    assert!(res.is_err());
}

#[tokio::test]
async fn test_when_node_stopped() {
    let ctx = Context::new(false);

    let cli = ctx.connect_to_cluster().await;

    put_and_get(&cli, 1).await;

    ctx.etcd_cluster.stop_node(1);

    put_and_get(&cli, 3).await;

    ctx.etcd_cluster.stop_node(2);

    expect_timeout(&cli);

    ctx.etcd_cluster.start_node(1);

    put_and_get(&cli, 3).await;

    ctx.etcd_cluster.stop_node(3);

    expect_timeout(&cli);

    ctx.etcd_cluster.start_node(2);

    put_and_get(&cli, 3).await;
}
