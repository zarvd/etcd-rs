use etcd_rs::*;

async fn reset(cli: &mut Client) -> Result<()> {
    cli.kv().delete(DeleteRequest::new(KeyRange::all())).await?;
    Ok(())
}

async fn compose(cli: &mut Client) -> Result<()> {
    reset(cli).await?;

    println!("start compose section =====>");

    let revision;
    {
        // init
        let mut resp = cli.kv().put(PutRequest::new("foo", "bar")).await?;
        revision = resp.take_header().unwrap().revision();

        for v in 0..10 {
            let _ = cli
                .kv()
                .put(PutRequest::new(format!("key-{}", v), format!("{}", v)))
                .await?;
        }
    }

    let txn = TxnRequest::new()
        .when_value(KeyRange::key("foo"), TxnCmp::Equal, "bar")
        .when_mod_revision(KeyRange::key("foo"), TxnCmp::Equal, revision as usize)
        .and_then(PutRequest::new("foo", "bar"))
        .and_then(RangeRequest::new(KeyRange::all()))
        .and_then(DeleteRequest::new(KeyRange::all()))
        .and_then(TxnRequest::new())
        .or_else(PutRequest::new("bar", "baz"));

    let mut txn_resp = cli.kv().txn(txn).await?;

    for op_resp in txn_resp.take_responses().into_iter() {
        match op_resp {
            TxnOpResponse::Put(resp) => {
                println!("put resp: {:?}", resp);
            }
            TxnOpResponse::Range(resp) => {
                println!("range resp: {:?}", resp);
            }
            TxnOpResponse::Delete(resp) => {
                println!("delete resp: {:?}", resp);
            }
            TxnOpResponse::Txn(resp) => {
                println!("txn resp: {:?}", resp);
            }
        }
    }
    println!("<===== end compose section");

    Ok(())
}

async fn cas(cli: &mut Client) -> Result<()> {
    reset(cli).await?;
    println!("start CAS section =====>");
    // init
    {
        cli.kv().put(PutRequest::new("foo", "bar")).await?;
    }

    // if foo == bar then set foo = baz
    let txn = TxnRequest::new()
        .when_value(KeyRange::key("foo"), TxnCmp::Equal, "bar")
        .and_then(PutRequest::new("foo", "baz"));

    let resp = cli.kv().txn(txn).await?;

    println!("txn resp = {:?}", resp);
    println!("<===== end CAS section");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::connect(ClientConfig {
        endpoints: vec![
            "http://127.0.0.1:12379".to_owned(),
            "http://127.0.0.1:22379".to_owned(),
            "http://127.0.0.1:32379".to_owned(),
        ],
        ..Default::default()
    })
    .await?;

    // Compare-and-Set
    if let Err(e) = cas(&mut client).await {
        println!("failed to execute CAS: {:?}", e);
    }

    // Compose
    if let Err(e) = compose(&mut client).await {
        println!("failed to execute CAS: {:?}", e);
    }

    Ok(())
}
