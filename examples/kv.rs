#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = etcd_rs::Client::new(etcd_rs::ClientConfig {
        endpoints: vec!["http://127.0.0.1:2379".to_owned()],
        auth: None,
    });

    let req = etcd_rs::PutRequest::new("foo", "bar");

    let resp = client.kv().put(req).await?;

    println!("{:?}", resp);

    Ok(())
}
