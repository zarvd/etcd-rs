use futures::StreamExt;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::timeout;

use etcd_rs::*;

#[macro_use]
mod support;
use crate::support::{Context, KVOp};

#[tokio::test]
async fn test_put_and_get_kv() {
    let ctx = Context::new(false);

    let cli = ctx.connect_to_cluster().await;
    let key = "foo";
    cli.kv()
        .put(PutRequest::new(key, "bar"))
        .await
        .expect("put kv");

    let mut resp = cli
        .kv()
        .range(RangeRequest::new(KeyRange::key(key)))
        .await
        .expect("range kv");

    assert_eq!(1, resp.count());
    let kvs = resp.take_kvs();
    assert_eq!("foo", kvs[0].key_str());
    assert_eq!("bar", kvs[0].value_str());
}

#[tokio::test]
async fn test_put_and_del_kv() {
    let ctx = Context::new(false);

    let cli = ctx.connect_to_cluster().await;
    let key = "foo";

    cli.kv()
        .put(PutRequest::new(key, "bar"))
        .await
        .expect("put kv");

    let resp = cli
        .kv()
        .delete(DeleteRequest::new(KeyRange::key(key)))
        .await
        .expect("delete kv");

    assert_eq!(1, resp.count_deleted());

    let resp = cli
        .kv()
        .range(RangeRequest::new(KeyRange::key(key)))
        .await
        .expect("range kv");

    assert_eq!(0, resp.count());
}

#[tokio::test]
async fn test_list_kv_by_prefix() {
    let ctx = Context::new(false);

    let cli = ctx.connect_to_cluster().await;

    let prefix = "42_";

    let mess_kvs = {
        let mut s = HashSet::new();
        s.insert(("41_foo1", "baz1"));
        s.insert(("43_foo1", "baz2"));
        s.insert(("abc", ""));
        s
    };
    for (k, v) in mess_kvs.iter() {
        cli.kv().put(PutRequest::new(*k, *v)).await.unwrap();
    }

    let expected_kvs = {
        let mut s = HashSet::new();
        s.insert(("42_foo1", "bar1"));
        s.insert(("42_foo2", "bar2"));
        s.insert(("42_foo3", "bar3"));
        s.insert(("42_baz1", "bar4"));
        s.insert(("42_baz2", "bar5"));
        s
    };
    for (k, v) in expected_kvs.iter() {
        cli.kv().put(PutRequest::new(*k, *v)).await.unwrap();
    }

    let mut resp = cli
        .kv()
        .range(RangeRequest::new(KeyRange::prefix(prefix)))
        .await
        .unwrap();

    assert_eq!(expected_kvs.len(), resp.count());

    let actual_kvs = resp.take_kvs();

    for kv in actual_kvs {
        assert!(expected_kvs.contains(&(kv.key_str(), kv.value_str())));
    }
}

#[tokio::test]
async fn test_watch() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    const PREFIX: &str = "prefix-test-watch";

    let mut tunnel = cli.watch_client().watch(KeyRange::prefix(PREFIX)).await;

    assert_watch_created!(tunnel);

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

    assert_ops_events!(ops, tunnel);
}

#[tokio::test]
async fn test_watch_multi() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    const PREFIX1: &str = "prefix-test-watch-multi1";
    const PREFIX2: &str = "prefix-test-watch-multi2";

    let mut tunnel1_1 = cli.watch_client().watch(KeyRange::prefix(PREFIX1)).await;
    let mut tunnel2 = cli.watch_client().watch(KeyRange::prefix(PREFIX2)).await;

    assert_watch_created!(tunnel1_1);
    assert_watch_created!(tunnel2);

    let ops_1: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo1".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX1, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX1, k)),
    })
    .collect();

    let ops_2: Vec<_> = vec![
        KVOp::Put("foo1".to_owned(), "bar1".to_owned()),
        KVOp::Put("foo2".to_owned(), "bar2".to_owned()),
        KVOp::Put("foo3".to_owned(), "bar3".to_owned()),
        KVOp::Put("foo4".to_owned(), "bar3".to_owned()),
        KVOp::Delete("foo1".to_owned()),
        KVOp::Delete("foo2".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX2, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX2, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops_1);
    apply_kv_ops!(cli, ops_2);

    assert_ops_events!(ops_1, tunnel1_1);
    assert_ops_events!(ops_2, tunnel2);

    let mut tunnel1_2 = cli.watch_client().watch(KeyRange::prefix(PREFIX1)).await;

    assert_watch_created!(tunnel1_2);

    let ops_1: Vec<_> = vec![
        KVOp::Put("foo4".to_owned(), "bar4".to_owned()),
        KVOp::Put("foo5".to_owned(), "bar5".to_owned()),
        KVOp::Put("foo6".to_owned(), "bar6".to_owned()),
        KVOp::Delete("foo4".to_owned()),
        KVOp::Delete("foo5".to_owned()),
    ]
    .into_iter()
    .map(|op| match op {
        KVOp::Put(k, v) => KVOp::Put(format!("{}-{}", PREFIX1, k), v),
        KVOp::Delete(k) => KVOp::Delete(format!("{}-{}", PREFIX1, k)),
    })
    .collect();

    apply_kv_ops!(cli, ops_1);

    assert_ops_events!(ops_1, tunnel1_1);
    assert_ops_events!(ops_1, tunnel1_2);
}
