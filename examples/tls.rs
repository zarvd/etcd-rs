use etcd_rs::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tonic::transport::{Certificate, ClientTlsConfig, Identity};

async fn put_and_get(client: &Client) -> Result<()> {
    println!("Put and get a key value pairs");

    let key = "foo";
    let value = "bar";

    {
        // Put a key-value pair
        let req = PutRequest::new(key, value);
        println!("Put and ff a key value pairs");

        let resp = client.kv().put(req).await?;

        println!("Put Response: {:?}", resp);
    }

    {
        // Get the key-value pair
        let req = RangeRequest::new(KeyRange::key(key));
        let resp = client.kv().range(req).await?;
        println!("Range Response: {:?}", resp);
    }

    {
        // Delete the key-valeu pair
        let req = DeleteRequest::new(KeyRange::key(key));
        let resp = client.kv().delete(req).await?;
        println!("Delete Response: {:?}", resp);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut ca: Vec<u8> = Vec::new();
    let mut cert: Vec<u8> = Vec::new();
    let mut key: Vec<u8> = Vec::new();

    File::open(Path::new("ca.pem"))
        .unwrap()
        .read_to_end(&mut ca)
        .unwrap();
    File::open(Path::new("cert.pem"))
        .unwrap()
        .read_to_end(&mut cert)
        .unwrap();
    File::open(Path::new("key.pem"))
        .unwrap()
        .read_to_end(&mut key)
        .unwrap();

    let tls = ClientTlsConfig::new();
    let tls = tls.ca_certificate(Certificate::from_pem(ca));
    let tls = tls.identity(Identity::from_pem(cert, key));

    let client = Client::connect(ClientConfig {
        endpoints: vec!["https://127.0.0.1:2379".to_owned()],
        tls: Some(tls),
        ..Default::default()
    })
    .await?;

    put_and_get(&client).await?;

    Ok(())
}
