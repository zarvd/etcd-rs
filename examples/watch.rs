use etcd_rs::{Client, ClientConfig, KeyRange, KeyValueOp, Result, WatchInbound, WatchOp};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Client::connect(ClientConfig::new([
        "http://127.0.0.1:12379".to_owned(),
        "http://127.0.0.1:22379".to_owned(),
        "http://127.0.0.1:32379".to_owned(),
    ]))
    .await?;

    let (mut stream, cancel) = cli
        .watch(KeyRange::prefix("foo"))
        .await
        .expect("watch by prefix");

    tokio::spawn(async move {
        cli.put(("foo1", "1")).await.expect("put kv");
        cli.put(("bar", "2")).await.expect("put kv");
        cli.put(("foo2", "3")).await.expect("put kv");
        cli.put(("bar", "4")).await.expect("put kv");
        cli.put(("foo2", "5")).await.expect("put kv");
        cli.delete("foo1").await.expect("delete kv");
        cli.delete("bar").await.expect("delete kv");

        cancel.cancel().await.expect("cancel watch");
    });

    loop {
        match stream.inbound().await {
            WatchInbound::Ready(resp) => {
                println!("receive event: {:?}", resp);
            }
            WatchInbound::Interrupted(e) => {
                eprintln!("encounter error: {:?}", e);
            }
            WatchInbound::Closed => {
                println!("watch stream closed");
                break;
            }
        }
    }

    Ok(())
}
