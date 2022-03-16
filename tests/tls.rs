use etcd_rs::*;

#[macro_use]
mod support;
use crate::support::Context;

#[tokio::test]
async fn test_put() {
    let ctx = Context::new(true);
    let cli = ctx.connect_to_cluster().await;

    cli.put(("foo", "bar")).await.expect("put kv");

    let resp = cli.get("foo").await.expect("get kv");
    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(resp.kvs[0].key_str(), "foo");
    assert_eq!(resp.kvs[0].value_str(), "bar");
}
