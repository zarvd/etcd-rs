mod grant;
mod keep_alive;
mod revoke;
pub use grant::{LeaseGrantRequest, LeaseGrantResponse};
pub use keep_alive::{LeaseKeepAliveRequest, LeaseKeepAliveResponse};
pub use revoke::{LeaseRevokeRequest, LeaseRevokeResponse};

use std::sync::{Arc, RwLock};

use tokio::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tonic::transport::Channel;

use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::client::LeaseClient;
use crate::Result;

/// LeaseKeepAliveTunnel is a reusable connection for `Lease Keep Alive` operation.
/// The underlying gRPC method is Bi-directional streaming.
struct LeaseKeepAliveTunnel {
    req_sender: UnboundedSender<LeaseKeepAliveRequest>,
    resp_receiver:
        Option<UnboundedReceiver<std::result::Result<LeaseKeepAliveResponse, tonic::Status>>>,
}

impl LeaseKeepAliveTunnel {
    fn new(mut client: LeaseClient<Channel>) -> Self {
        let (req_sender, mut req_receiver) = unbounded_channel::<LeaseKeepAliveRequest>();
        let (mut resp_sender, resp_receiver) =
            unbounded_channel::<std::result::Result<LeaseKeepAliveResponse, tonic::Status>>();

        let request = tonic::Request::new(async_stream::stream! {
            while let Some(req) = req_receiver.next().await {
                let pb: etcdserverpb::LeaseKeepAliveRequest = req.into();
                yield pb;
            }
        });

        // monitor inbound watch response and transfer to the receiver
        tokio::spawn(async move {
            let mut inbound = client.lease_keep_alive(request).await.unwrap().into_inner();

            loop {
                let resp = inbound.message().await;
                match resp {
                    Ok(Some(resp)) => {
                        resp_sender.send(Ok(From::from(resp))).await.unwrap();
                    }
                    Ok(None) => {
                        return;
                    }
                    Err(e) => {
                        resp_sender.send(Err(e)).await.unwrap();
                    }
                };
            }
        });

        Self {
            req_sender,
            resp_receiver: Some(resp_receiver),
        }
    }

    fn take_resp_receiver(
        &mut self,
    ) -> UnboundedReceiver<std::result::Result<LeaseKeepAliveResponse, tonic::Status>> {
        self.resp_receiver
            .take()
            .expect("take the unique watch response receiver")
    }
}

#[derive(Clone)]
pub struct Lease {
    client: LeaseClient<Channel>,
    keep_alive_tunnel: Arc<RwLock<LeaseKeepAliveTunnel>>,
}

/// Lease client
impl Lease {
    pub(crate) fn new(client: LeaseClient<Channel>) -> Self {
        let keep_alive_tunnel = Arc::new(RwLock::new(LeaseKeepAliveTunnel::new(client.clone())));
        Self {
            client,
            keep_alive_tunnel,
        }
    }

    pub async fn grant(&mut self, req: LeaseGrantRequest) -> Result<LeaseGrantResponse> {
        let resp = self
            .client
            .lease_grant(tonic::Request::new(req.into()))
            .await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn revoke(&mut self, req: LeaseRevokeRequest) -> Result<LeaseRevokeResponse> {
        let resp = self
            .client
            .lease_revoke(tonic::Request::new(req.into()))
            .await?;

        Ok(From::from(resp.into_inner()))
    }

    /// Fetch keep alive response stream
    pub fn keep_alive_responses(
        &mut self,
    ) -> impl Stream<Item = std::result::Result<LeaseKeepAliveResponse, tonic::Status>> {
        self.keep_alive_tunnel.write().unwrap().take_resp_receiver()
    }

    pub async fn keep_alive(&mut self, req: LeaseKeepAliveRequest) {
        self.keep_alive_tunnel
            .write()
            .unwrap()
            .req_sender
            .send(req)
            .await
            .unwrap();
    }
}
