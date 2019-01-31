use std::sync::Arc;

use futures::{Future, Sink, Stream};

use crate::client::Inner;
use crate::lease::{
    GrantRequest, GrantResponse, KeepAlive, KeepAliveRequest, KeepAliveResponse, RevokeRequest,
    RevokeResponse, TtlRequest, TtlResponse,
};
use crate::Error;

pub struct LeaseClient {
    inner: Arc<Inner>,
}

impl LeaseClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub fn keep_alive_once(
        &self,
        req: KeepAliveRequest,
    ) -> impl Future<Item = KeepAliveResponse, Error = Error> {
        let (sink, receiver) = self.inner.lease.lease_keep_alive().unwrap();

        sink.send((req.into(), Default::default()))
            .and_then(move |_| receiver.and_then(|resp| Ok(From::from(resp))))
            .or_else(|e| Err(Error::GrpcFailure(e)))
    }

    pub fn grant(&self, req: GrantRequest) -> impl Future<Item = GrantResponse, Error = Error> {
        self.inner
            .lease
            .lease_grant_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }

    pub fn revoke(&self, req: RevokeRequest) -> impl Future<Item = RevokeResponse, Error = Error> {
        self.inner
            .lease
            .lease_revoke_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }

    pub fn ttl(&self, req: TtlRequest) -> impl Future<Item = TtlResponse, Error = Error> {
        self.inner
            .lease
            .lease_time_to_live_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }
}
