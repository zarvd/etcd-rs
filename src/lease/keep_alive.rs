use futures::{Async, AsyncSink, Poll, Sink, StartSend};

use crate::proto::rpc;
use crate::Error;
use crate::ResponseHeader;

#[derive(Clone)]
pub struct KeepAliveRequest {
    id: i64,
}

impl KeepAliveRequest {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl Into<rpc::LeaseKeepAliveRequest> for KeepAliveRequest {
    fn into(self) -> rpc::LeaseKeepAliveRequest {
        let mut req = rpc::LeaseKeepAliveRequest::new();
        req.set_ID(self.id);
        req
    }
}

#[derive(Debug)]
pub struct KeepAliveResponse {
    resp: rpc::LeaseKeepAliveResponse,
}

impl KeepAliveResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }

    pub fn id(&self) -> i64 {
        self.resp.get_ID()
    }

    pub fn ttl(&self) -> i64 {
        self.resp.get_TTL()
    }
}

impl From<rpc::LeaseKeepAliveResponse> for KeepAliveResponse {
    fn from(resp: rpc::LeaseKeepAliveResponse) -> Self {
        Self { resp }
    }
}

pub struct KeepAlive {
    sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
    req: rpc::LeaseKeepAliveRequest,
}

impl KeepAlive {
    pub fn new(
        sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
        req: KeepAliveRequest,
    ) -> Self {
        Self {
            sender,
            req: req.into(),
        }
    }
}

impl Sink for KeepAlive {
    type SinkItem = ();
    type SinkError = Error;

    fn start_send(&mut self, _item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        match self
            .sender
            .start_send((self.req.clone(), Default::default()))
        {
            Ok(AsyncSink::Ready) => Ok(AsyncSink::Ready),
            Ok(AsyncSink::NotReady(_)) => Ok(AsyncSink::NotReady(())),
            Err(e) => Err(Error::GrpcFailure(e)),
        }
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        match self.sender.poll_complete() {
            Ok(Async::Ready(())) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(Error::GrpcFailure(e)),
        }
    }
}
