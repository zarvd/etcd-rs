use tokio::prelude::*;

use etcd_rs::*;

async fn watch(client: &Client) -> Result<()> {
    println!("watch key value modification");

    {
        // print out all received watch responses
        let mut inbound = client.watch().responses();
        tokio::spawn(async move {
            loop {
                let resp = inbound.next().await.unwrap();
                println!("watch response: {:?}", resp);
            }
        });
    }

    client
        .watch()
        .watch(WatchRequest::create(KeyRange::key("foo")))
        .await;

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
