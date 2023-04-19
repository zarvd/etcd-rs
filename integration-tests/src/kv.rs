use std::time::Duration;

use tonic::Code;

use etcd_rs::*;

use crate::support::Context;

#[tokio::test]
async fn test_put_error() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let resp = cli.put(("", "bar")).await;
    match resp {
        Err(Error::Response(status)) => {
            assert_eq!(status.code(), Code::InvalidArgument);
            assert_eq!(status.message(), "etcdserver: key is not provided")
        }
        _ => unreachable!(),
    }

    // hard code max in server-side
    let resp = cli
        .put(("foo", "x".repeat((1.5 * (1024 * 1024) as f64) as usize)))
        .await;
    match resp {
        Err(Error::Response(status)) => {
            assert_eq!(status.code(), Code::InvalidArgument);
            assert_eq!(status.message(), "etcdserver: request is too large")
        }
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn test_put_with_lease() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let (key, value) = ("foo", "bar");

    let lease = cli
        .grant_lease(Duration::from_secs(10))
        .await
        .expect("grant lease");

    cli.put(PutRequest::from((key, value)).lease(lease.id))
        .await
        .expect("put kv with lease");

    let resp = cli.get(key).await.expect("get kv");
    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(key, resp.kvs[0].key_str());
    assert_eq!(value, resp.kvs[0].value_str());
    assert_eq!(lease.id, resp.kvs[0].lease)
}

#[tokio::test]
async fn test_put_with_ignore_value() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let (key, value) = ("foo", "bar");

    match cli.put(PutRequest::from((key, "")).ignore_value()).await {
        Err(Error::Response(status)) => {
            assert_eq!(status.code(), Code::InvalidArgument);
            assert_eq!(status.message(), "etcdserver: key not found")
        }
        _ => unreachable!(),
    }

    cli.put((key, value)).await.expect("put kv");

    cli.put(PutRequest::from((key, "")).ignore_value())
        .await
        .expect("put kv with ignore value");

    let resp = cli.get(key).await.expect("get kv");
    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(key, resp.kvs[0].key_str());
    assert_eq!(value, resp.kvs[0].value_str());
}

#[tokio::test]
async fn test_put_with_ignore_lease() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let (key, value) = ("foo", "bar");

    match cli.put(PutRequest::from((key, "")).ignore_lease()).await {
        Err(Error::Response(status)) => {
            assert_eq!(status.code(), Code::InvalidArgument);
            assert_eq!(status.message(), "etcdserver: key not found")
        }
        _ => unreachable!(),
    }

    let lease = cli
        .grant_lease(Duration::from_secs(10))
        .await
        .expect("grant lease");

    cli.put(PutRequest::from((key, value)).lease(lease.id))
        .await
        .expect("put kv with lease");

    cli.put(PutRequest::from((key, "bar1")).ignore_lease())
        .await
        .expect("put kv with ignore lease");

    let resp = cli.get(key).await.expect("get kv");
    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(key, resp.kvs[0].key_str());
    assert_eq!("bar1", resp.kvs[0].value_str());
    assert_eq!(lease.id, resp.kvs[0].lease)
}

#[tokio::test]
async fn test_get_all() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let kvs = vec![
        ("a", "a1"),
        ("b", "b1"),
        ("c", "c1"),
        ("c", "c2"),
        ("c", "c3"),
        ("foo", "foo1"),
        ("foo/abc", "foo/abc1"),
        ("fop", "fop1"),
    ];

    for (k, v) in kvs {
        cli.put((k, v))
            .await
            .expect(&format!("put kv: ({}, {})", k, v));
    }

    let resp = cli.get_all().await.expect("get all key-value");

    assert_eq!(resp.count, 6);

    assert_eq!(
        resp.kvs,
        vec![
            KeyValue {
                key: "a".into(),
                value: "a1".into(),
                create_revision: 2,
                mod_revision: 2,
                version: 1,
                lease: 0
            },
            KeyValue {
                key: "b".into(),
                value: "b1".into(),
                create_revision: 3,
                mod_revision: 3,
                version: 1,
                lease: 0
            },
            KeyValue {
                key: "c".into(),
                value: "c3".into(),
                create_revision: 4,
                mod_revision: 6,
                version: 3,
                lease: 0
            },
            KeyValue {
                key: "foo".into(),
                value: "foo1".into(),
                create_revision: 7,
                mod_revision: 7,
                version: 1,
                lease: 0
            },
            KeyValue {
                key: "foo/abc".into(),
                value: "foo/abc1".into(),
                create_revision: 8,
                mod_revision: 8,
                version: 1,
                lease: 0
            },
            KeyValue {
                key: "fop".into(),
                value: "fop1".into(),
                create_revision: 9,
                mod_revision: 9,
                version: 1,
                lease: 0
            }
        ]
    );
}

#[tokio::test]
async fn test_delete_all() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    let kvs = vec![
        ("a", "a1"),
        ("b", "b1"),
        ("c", "c1"),
        ("c", "c2"),
        ("c", "c3"),
        ("foo", "foo1"),
        ("foo/abc", "foo/abc1"),
        ("fop", "fop1"),
    ];

    for (k, v) in kvs {
        cli.put((k, v))
            .await
            .expect(&format!("put kv: ({}, {})", k, v));
    }

    cli.delete_all().await.expect("delete all key-value");

    let resp = cli.get_all().await.expect("get all key-value");
    assert_eq!(resp.count, 0);
    assert!(resp.kvs.is_empty());
}

#[tokio::test]
async fn test_compact_error() {
    let ctx = Context::new(false);
    let cli = ctx.connect_to_cluster().await;

    for _ in 0..5 {
        cli.put(("foo", "bar")).await.expect("put key-value");
    }

    cli.compact(6).await.expect("compact with current revision");
    cli.compact(6)
        .await
        .expect_err("compact with compacted revision");
    cli.compact(42)
        .await
        .expect_err("compact with future revision");
}
