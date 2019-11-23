use std::time::Duration;

use tokio::prelude::*;

use etcd_rs::*;

async fn grant_lease(client: &Client) -> Result<()> {
    println!("grant lease");

    let key = "foo";

    {
        // watch key modification
        let mut inbound = client.watch().responses();
        tokio::spawn(async move {
            loop {
                let resp = inbound.next().await.unwrap();
                println!("watch response: {:?}", resp);
            }
        });
        client
            .watch()
            .watch(WatchRequest::create(KeyRange::key(key)))
            .await;
    }

    let lease = client
        .lease()
        .grant(LeaseGrantRequest::new(Duration::from_secs(3)))
        .await?;

    client
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

async fn keep_alive_lease(client: &Client) -> Result<()> {
    println!("grant lease and keep alive");

    let key = "foo";

    {
        // watch key modification
        let mut inbound = client.watch().responses();
        tokio::spawn(async move {
            loop {
                let resp = inbound.next().await.unwrap();
                println!("watch response: {:?}", resp);
            }
        });
        client
            .watch()
            .watch(WatchRequest::create(KeyRange::key(key)))
            .await;
    }

    // grant lease
    let lease = client
        .lease()
        .grant(LeaseGrantRequest::new(Duration::from_secs(3)))
        .await?;

    let lease_id = lease.id();

    {
        // watch keep alive event
        let mut inbound = client.lease().keep_alive_responses();
        tokio::spawn(async move {
            loop {
                let resp = inbound.next().await.unwrap();
                println!("=====>");
                println!("keep alive response: {:?}", resp);
            }
        });
    }

    // set lease for key
    client
        .kv()
        .put({
            let mut req = PutRequest::new(key, "bar");
            req.set_lease(lease_id);

            req
        })
        .await?;

    {
        // keep alive the lease every 1 second
        let client = client.clone();

        use tokio::timer::Interval;
        let mut interval = Interval::new_interval(Duration::from_secs(1));

        loop {
            interval.next().await;
            client
                .lease()
                .keep_alive(LeaseKeepAliveRequest::new(lease_id))
                .await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::connect(ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
    }).await?;

    // grant_lease(&client).await?;
    keep_alive_lease(&client).await?;

    Ok(())
}
