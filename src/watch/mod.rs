mod watch;
pub use watch::{WatchRequest, WatchResponse};

use std::sync::{Arc, RwLock};

use tokio::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tonic::transport::Channel;

use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::client::WatchClient;

/// WatchTunnel is a reusable connection for `Watch` operation
/// underlying gRPC method is Bi-directional streaming
struct WatchTunnel {
    req_sender: UnboundedSender<WatchRequest>,
    resp_receiver: Option<UnboundedReceiver<Result<WatchResponse, tonic::Status>>>,
}

impl WatchTunnel {
    fn new(mut client: WatchClient<Channel>) -> Self {
        let (req_sender, mut req_receiver) = unbounded_channel::<WatchRequest>();
        let (mut resp_sender, resp_receiver) =
            unbounded_channel::<Result<WatchResponse, tonic::Status>>();

        let request = tonic::Request::new(async_stream::stream! {
            while let Some(req) = req_receiver.next().await {
                let pb: etcdserverpb::WatchRequest = req.into();
                yield pb;
            }
        });

        // monitor inbound watch response and transfer to the receiver
        tokio::spawn(async move {
            let mut inbound = client.watch(request).await.expect("fuck 1").into_inner();

            loop {
                let resp = inbound.message().await;
                match resp {
                    Ok(Some(resp)) => {
                        resp_sender
                            .send(Ok(From::from(resp)))
                            .await
                            .expect("fuck 2");
                    }
                    Ok(None) => {
                        return;
                    }
                    Err(e) => {
                        resp_sender.send(Err(e)).await.expect("fuck 3");
                    }
                };
            }
        });

        Self {
            req_sender,
            resp_receiver: Some(resp_receiver),
        }
    }

    fn take_resp_receiver(&mut self) -> UnboundedReceiver<Result<WatchResponse, tonic::Status>> {
        self.resp_receiver
            .take()
            .expect("take the unique watch response receiver")
    }
}

#[derive(Clone)]
pub struct Watch {
    client: WatchClient<Channel>,
    tunnel: Arc<RwLock<WatchTunnel>>,
}

impl Watch {
    pub(crate) fn new(client: WatchClient<Channel>) -> Self {
        let tunnel = Arc::new(RwLock::new(WatchTunnel::new(client.clone())));

        Self { client, tunnel }
    }

    /// Fetch response stream
    pub fn response(&mut self) -> impl Stream<Item = Result<WatchResponse, tonic::Status>> {
        self.tunnel.write().expect("fuck").take_resp_receiver()
    }

    /// Emit request
    pub async fn watch(&mut self, req: WatchRequest) {
        self.tunnel
            .write()
            .expect("fuck")
            .req_sender
            .send(req)
            .await
            .expect("fuck");
    }
}
