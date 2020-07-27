use tokio::stream::StreamExt;
use tokio::time::{delay_for, Duration};

use etcd_rs::*;

async fn watch(client: &mut Client) -> Result<()> {
    println!("watch key value modification");

    {
        let mut inbound = client.watch(KeyRange::key("foo")).await;

        // print out all received watch responses
        tokio::spawn(async move {
            while let Some(resp) = inbound.next().await {
                println!("watch foo response: {:?}", resp);
            }
        })
    };

    {
        let mut inbound = client.watch(KeyRange::key("foo2")).await;

        tokio::spawn(async move {
            while let Some(resp) = inbound.next().await {
                println!("watch foo2 response: {:?}", resp);
            }
        })
    };

    client.kv().put(PutRequest::new("foo", "bar")).await?;
    client.kv().put(PutRequest::new("foo2", "baz")).await?;

    delay_for(Duration::from_millis(1000)).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect(ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
        tls: None,
    })
    .await?;

    watch(&mut client).await?;

    client.shutdown().await?;

    Ok(())
}
