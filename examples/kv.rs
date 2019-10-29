use etcd_rs::*;

async fn list_prefix(client: &Client) -> Result<()> {
    println!("List key value pairs with prefix");

    let prefix = "42_";

    client.kv().put(PutRequest::new("41_foo1", "baz1")).await?;
    {
        // Put some key-value pairs
        client.kv().put(PutRequest::new("42_foo1", "baz1")).await?;
        client.kv().put(PutRequest::new("42_foo2", "baz2")).await?;
        client.kv().put(PutRequest::new("42_bar1", "baz3")).await?;
        client.kv().put(PutRequest::new("42_bar2", "baz4")).await?;
    }

    {
        // List key-value pairs with prefix
        let req = RangeRequest::prefix(prefix);
        let mut resp = client.kv().range(req).await?;

        println!("Range Response: {:?}", resp);
        for kv in resp.take_kvs() {
            println!("{:?} -> {:?}", kv.key_str(), kv.value_str());
        }
    }

    {
        // Delete key-valeu pairs with prefix
        let req = DeleteRequest::prefix(prefix);
        let resp = client.kv().delete(req).await?;
        println!("Delete Response: {:?}", resp);
    }

    Ok(())
}

async fn list_all(client: &Client) -> Result<()> {
    println!("List all key value pairs");
    {
        // Put some key-value pairs
        client.kv().put(PutRequest::new("foo1", "baz1")).await?;
        client.kv().put(PutRequest::new("foo2", "baz2")).await?;
        client.kv().put(PutRequest::new("bar1", "baz3")).await?;
        client.kv().put(PutRequest::new("bar2", "baz4")).await?;
    }

    {
        // List all key-value pairs
        let req = {
            let mut req = RangeRequest::all();
            req.set_limit(4); // Only returns 4 key-value pairs
            req
        };
        let resp = client.kv().range(req).await?;
        println!("Range Response: {:?}", resp);
    }

    {
        // Delete all key-valeu pairs
        let req = DeleteRequest::all();
        let resp = client.kv().delete(req).await?;
        println!("Delete Response: {:?}", resp);
    }

    Ok(())
}

async fn put_and_get(client: &Client) -> Result<()> {
    println!("Put and get a key value pairs");

    let key = "foo";
    let value = "bar";

    {
        // Put a key-value pair
        let req = PutRequest::new(key, value);
        let resp = client.kv().put(req).await?;

        println!("Put Response: {:?}", resp);
    }

    {
        // Get the key-value pair
        let req = RangeRequest::key(key);
        let resp = client.kv().range(req).await?;
        println!("Range Response: {:?}", resp);
    }

    {
        // Delete the key-valeu pair
        let req = DeleteRequest::key(key);
        let resp = client.kv().delete(req).await?;
        println!("Delete Response: {:?}", resp);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
    });

    put_and_get(&client).await?;
    list_all(&client).await?;
    list_prefix(&client).await?;

    Ok(())
}
