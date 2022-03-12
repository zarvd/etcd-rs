use std::time::Duration;

use etcd_rs::{Client, ClientConfig, KeyRange, KeyValueOp, LeaseOp, PutRequest, Result};

async fn put(cli: &Client) -> Result<()> {
    cli.put(("foo", "bar")).await.expect("put kv");
    let resp = cli.get("foo").await.expect("get kv");

    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(resp.kvs[0].key_str(), "foo");
    assert_eq!(resp.kvs[0].value_str(), "bar");

    Ok(())
}

async fn put_with_lease(cli: &Client) -> Result<()> {
    let lease = cli
        .grant_lease(Duration::from_secs(10))
        .await
        .expect("grant lease");
    cli.put(PutRequest::new("foo", "bar").lease(lease.id))
        .await
        .expect("put kv with lease");

    Ok(())
}

async fn get(cli: &Client) -> Result<()> {
    cli.get(KeyRange::range("start", "end"))
        .await
        .expect("get range kvs");
    cli.get_range("start", "end").await.expect("get range kvs");

    cli.get(KeyRange::all()).await.expect("get all kvs");
    cli.get_all().await.expect("get all kvs");

    cli.get(KeyRange::prefix("foo"))
        .await
        .expect("get by prefix");
    cli.get_by_prefix("foo").await.expect("get by prefix");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Client::connect(ClientConfig::new([
        "http://127.0.0.1:12379".into(),
        "http://127.0.0.1:22379".into(),
        "http://127.0.0.1:32379".into(),
    ]))
    .await?;

    put(&cli).await?;
    put_with_lease(&cli).await?;
    get(&cli).await?;

    Ok(())
}
