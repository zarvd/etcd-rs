use std::collections::HashSet;

use etcd_rs::*;

mod support;
use crate::support::Context;

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

    let mut resp = cli
        .kv()
        .delete(DeleteRequest::new(KeyRange::key(key)))
        .await
        .expect("delete kv");

    assert_eq!(1, resp.count_deleted());

    let mut resp = cli
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
