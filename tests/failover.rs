use std::time::Duration;

use rand::Rng;
use tokio::time::timeout;

use etcd_rs::*;

#[macro_use]
mod support;
use crate::support::{Context, KVOp};

async fn put_and_get(cli: &Client, retry: usize) {
    for _ in 0..=retry {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let k = format!("key-{}", rand::thread_rng().gen::<u64>());
        let v = rand::thread_rng().gen::<u64>().to_string();
        let r = cli.put(PutRequest::new(k.clone(), v.clone())).await;
        if let Some(e) = r.err() {
            eprintln!("failed to put kv (will retry): {:?}", e);
            continue;
        }

        let r = cli.get(k).await;

        match r {
            Ok(resp) => {
                assert_eq!(1, resp.count);
                assert_eq!(&v, resp.kvs[0].value_str());

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

        let res = cli.put(PutRequest::new("foo", "bar")).await; // FIXME check specified error
        assert!(res.is_err(), "resp = {:?}", res);
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

    ctx.etcd_cluster.print_status();
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

    ctx.etcd_cluster.print_status();
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

    let mut stream = cli
        .watch(KeyRange::prefix(PREFIX))
        .await
        .expect("watch created");

    ctx.etcd_cluster.stop_node(1);
    ctx.etcd_cluster.stop_node(2);
    ctx.etcd_cluster.stop_node(3);

    {
        let mut interrupted = false;

        for _ in 0..10 {
            let x = timeout(Duration::from_secs(1), stream.inbound()).await;
            match x {
                Ok(etcd_rs::WatchInbound::Interrupted(_)) => {
                    interrupted = true;
                    break;
                }
                Ok(etcd_rs::WatchInbound::Closed) => {
                    panic!("should not close watch stream");
                }
                Err(e) => {
                    println!("timeout: {:?}", e);
                }
                Ok(v) => {
                    panic!("should not reach here: {:?}", v)
                }
            }
        }

        assert!(interrupted);
    }

    expect_timeout(&cli).await;

    ctx.etcd_cluster.start_node(1);
    ctx.etcd_cluster.start_node(2);
    ctx.etcd_cluster.start_node(3);

    tokio::time::sleep(Duration::from_secs(2)).await;

    put_and_get(&cli, 5).await; // re-connect to cluster
    put_and_get(&cli, 0).await;
    put_and_get(&cli, 0).await;

    let mut stream = cli
        .watch(KeyRange::prefix(PREFIX))
        .await
        .expect("watch created");

    let ops: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo3".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops);

    stream.cancel().await;
    assert_ops_events!(ops, stream);
}
