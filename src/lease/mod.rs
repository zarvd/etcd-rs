//! Leases are a mechanism for detecting client liveness. The cluster grants leases with a time-to-live. A lease expires if the etcd cluster does not receive a keepAlive within a given TTL period.
//!
//! # Examples
//!
//! Grant lease and keep lease alive

mod grant;
mod keep_alive;
mod revoke;
mod time_to_live;

pub use grant::{LeaseGrantRequest, LeaseGrantResponse};
pub use keep_alive::{LeaseKeepAliveRequest, LeaseKeepAliveResponse};
pub use revoke::{LeaseRevokeRequest, LeaseRevokeResponse};
pub use time_to_live::{LeaseTimeToLiveRequest, LeaseTimeToLiveResponse};

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tonic::Streaming;

use crate::Result;

pub type LeaseId = i64;

#[async_trait]
pub trait LeaseOp {
    async fn grant_lease<R>(&self, req: R) -> Result<LeaseGrantResponse>
    where
        R: Into<LeaseGrantRequest> + Send;

    async fn revoke<R>(&self, req: R) -> Result<LeaseRevokeResponse>
    where
        R: Into<LeaseRevokeRequest> + Send;

    async fn keep_alive_for(&self, lease_id: LeaseId) -> Result<LeaseKeepAlive>;

    async fn time_to_live<R>(&self, req: R) -> Result<LeaseTimeToLiveResponse>
    where
        R: Into<LeaseTimeToLiveRequest> + Send;
}

pub struct LeaseKeepAlive {
    id: LeaseId,
    req_tx: Sender<crate::proto::etcdserverpb::LeaseKeepAliveRequest>,
    resp_rx: Streaming<crate::proto::etcdserverpb::LeaseKeepAliveResponse>,
}

impl LeaseKeepAlive {
    pub(crate) fn new(
        id: LeaseId,
        req_tx: Sender<crate::proto::etcdserverpb::LeaseKeepAliveRequest>,
        resp_rx: Streaming<crate::proto::etcdserverpb::LeaseKeepAliveResponse>,
    ) -> Self {
        Self {
            id,
            req_tx,
            resp_rx,
        }
    }

    #[inline]
    pub fn lease_id(&mut self) -> LeaseId {
        self.id
    }

    pub async fn keep_alive(&mut self) -> Result<Option<LeaseKeepAliveResponse>> {
        let req = LeaseKeepAliveRequest::new(self.lease_id());

        self.req_tx
            .send(req.into())
            .await
            .expect("emit keep alive request to channel");

        Ok(match self.resp_rx.message().await? {
            Some(resp) => Some(resp.into()),
            None => None,
        })
    }
}
