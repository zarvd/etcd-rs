use std::time::Duration;

use futures::StreamExt;
use rand::Rng;

use etcd_rs::*;

mod support;
use crate::support::Context;

async fn put_and_get(cli: &Client, retry: usize) {
    for _ in 0..=retry {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let k = format!("key-{}", rand::thread_rng().gen::<u64>());
        let v = rand::thread_rng().gen::<u64>().to_string();
        let r = cli.kv().put(PutRequest::new(k.clone(), v.clone())).await;
        if let Some(e) = r.err() {
            eprintln!("failed to put kv (will retry): {:?}", e);
            continue;
        }

        let r = cli.kv().range(RangeRequest::new(KeyRange::key(k))).await;

        match r {
            Ok(mut resp) => {
                assert_eq!(1, resp.count());
                let kvs = resp.take_kvs();
                assert_eq!(&v, kvs[0].value_str());

                return;
            }
            Err(e) => {
                eprintln!("failed to range kv (will retry): {:?}", e);
            }
        }
    }

    unreachable!();
}

async fn expect_timeout(cli: &Client) {
    for _ in 0..3 {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let res = cli.kv().put(PutRequest::new("foo", "bar")).await; // FIXME check specified error
        assert!(res.is_err());
    }
}

#[tokio::test]
async fn test_kv_when_node_stopped() {
    let ctx = Context::new(false);

    let cli = ctx.connect_to_cluster().await;

    put_and_get(&cli, 0).await;

    ctx.etcd_cluster.stop_node(1);

    put_and_get(&cli, 5).await;

    ctx.etcd_cluster.stop_node(2);

    expect_timeout(&cli).await;

    ctx.etcd_cluster.start_node(1);

    put_and_get(&cli, 5).await;

    ctx.etcd_cluster.stop_node(3);

    expect_timeout(&cli).await;

    ctx.etcd_cluster.start_node(2);

    put_and_get(&cli, 5).await;
}

#[tokio::test]
async fn test_kv_when_cluster_down() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    put_and_get(&cli, 0).await;

    ctx.etcd_cluster.stop_node(1);
    ctx.etcd_cluster.stop_node(2);
    ctx.etcd_cluster.stop_node(3);

    expect_timeout(&cli).await;

    ctx.etcd_cluster.start_node(1);
    ctx.etcd_cluster.start_node(2);
    ctx.etcd_cluster.start_node(3);

    put_and_get(&cli, 5).await;
    put_and_get(&cli, 0).await;
    put_and_get(&cli, 0).await;
}

#[tokio::test]
async fn test_watch_when_cluster_down() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    const PREFIX: &str = "prefix-";

    let inbound = cli.watch(KeyRange::prefix(PREFIX)).await;

    assert!(inbound.is_ok());
    let mut inbound = inbound.unwrap();

    cli.kv()
        .put(PutRequest::new(format!("{}-foo", PREFIX), "1"))
        .await
        .unwrap();

    ctx.etcd_cluster.stop_node(1);
    ctx.etcd_cluster.stop_node(2);
    ctx.etcd_cluster.stop_node(3);

    assert!(inbound.next().await.is_some());
    while let Some(_) = inbound.next().await {
        // skipped
    }
    assert!(inbound.next().await.is_none());

    tokio::time::sleep(Duration::from_secs(3)).await;

    ctx.etcd_cluster.start_node(1);
    ctx.etcd_cluster.start_node(2);
    ctx.etcd_cluster.start_node(3);

    put_and_get(&cli, 5).await; // re-connect to cluster
    put_and_get(&cli, 0).await;
    put_and_get(&cli, 0).await;

    let mut inbound = cli.watch(KeyRange::prefix(PREFIX)).await.unwrap();

    cli.kv()
        .put(PutRequest::new(format!("{}-bar", PREFIX), "2"))
        .await
        .unwrap();

    assert!(inbound.next().await.is_some());
}
