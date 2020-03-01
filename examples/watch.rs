use tokio::stream::StreamExt;

use etcd_rs::*;

async fn watch(client: &Client) -> Result<()> {
    println!("watch key value modification");

    {
        let mut inbound = client.watch(KeyRange::key("foo")).await;

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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::connect(ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
    })
    .await?;

    watch(&client).await?;

    Ok(())
}
