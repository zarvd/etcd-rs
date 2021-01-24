use futures::StreamExt;

use etcd_rs::*;
use tokio::time::Duration;

async fn watch(client: &Client) -> Result<()> {
    println!("watch key value modification");

    {
        let mut inbound = client.watch(KeyRange::key("foo")).await.unwrap();

        // print out all received watch responses
        tokio::spawn(async move {
            while let Some(resp) = inbound.next().await {
                println!("watch response: {:?}", resp);
            }
        });
    }

    let key = "foo";
    client.kv().put(PutRequest::new(key, "bar")).await?;
    client.kv().put(PutRequest::new(key, "baz")).await?;
    client
        .kv()
        .delete(DeleteRequest::new(KeyRange::key(key)))
        .await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::connect(ClientConfig {
        endpoints: vec![
            "http://127.0.0.1:12379".to_owned(),
            "http://127.0.0.1:22379".to_owned(),
            "http://127.0.0.1:32379".to_owned(),
        ],
        auth: None,
        tls: None,
    })
    .await?;

    watch(&client).await?;

    client.shutdown().await?;

    Ok(())
}
