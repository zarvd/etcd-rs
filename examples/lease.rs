use std::time::Duration;

use futures::future::FutureExt;
use tokio::stream::StreamExt;

use etcd_rs::*;

type ArcClient = Arc<RwLock<Client>>;
use std::sync::Arc;
use tokio::sync::RwLock;

async fn grant_lease(client: ArcClient) -> Result<()> {
    println!("grant lease");

    let key = "foo";

    {
        // watch key modification
        let mut inbound = client.write().await.watch(KeyRange::key(key)).await;
        tokio::spawn(async move {
            while let Some(resp) = inbound.next().await {
                println!("watch response: {:?}", resp);
            }
        });
    }

    let lease = client
        .read()
        .await
        .lease()
        .grant(LeaseGrantRequest::new(Duration::from_secs(3)))
        .await?;

    client
        .read()
        .await
        .kv()
        .put({
            let mut req = PutRequest::new(key, "bar");
            req.set_lease(lease.id());

            req
        })
        .await?;

    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}

async fn keep_alive_lease(client: ArcClient) -> Result<()> {
    println!("grant lease and keep alive");

    let key = "foo";

    {
        // watch key modification
        let mut inbound = client.write().await.watch(KeyRange::key(key)).await;
        tokio::spawn(async move {
            while let Some(resp) = inbound.next().await {
                println!("watch response: {:?}", resp);
            }
        });
    }

    // grant lease
    let lease = client
        .write()
        .await
        .lease()
        .grant(LeaseGrantRequest::new(Duration::from_secs(3)))
        .await?;

    let lease_id = lease.id();

    {
        // watch keep alive event
        let mut inbound = client.write().await.lease().keep_alive_responses().await;
        tokio::spawn(async move {
            loop {
                match inbound.next().await {
                    Some(resp) => {
                        println!("=====>");
                        println!("keep alive response: {:?}", resp);
                    }
                    None => {
                        break;
                    }
                }
            }
        });
    }

    // set lease for key
    client
        .write()
        .await
        .kv()
        .put({
            let mut req = PutRequest::new(key, "bar");
            req.set_lease(lease_id);

            req
        })
        .await?;

    {
        // keep alive the lease every 1 second
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            client
                .write()
                .await
                .lease()
                .keep_alive(LeaseKeepAliveRequest::new(lease_id))
                .await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::connect(ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
        tls: None,
    })
    .await?;

    let client = Arc::new(RwLock::new(client));

    {
        let client = client.clone();
        grant_lease(client).await?;
    }

    {
        let client = client.clone();
        tokio::task::spawn(async move { keep_alive_lease(client).await });
    }

    {
        tokio::signal::ctrl_c()
            .then(|_| async { client.write().await.shutdown().await })
            .await?;
    }

    Ok(())
}
