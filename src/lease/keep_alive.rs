use futures::{Async, AsyncSink, Poll, Sink, Stream};

use crate::proto::rpc;
use crate::Error;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct KeepAliveRequest {
    id: i64,
}

impl KeepAliveRequest {
    pub fn new(id: i64) -> Self {
        KeepAliveRequest { id }
    }
}

impl Into<rpc::LeaseKeepAliveRequest> for KeepAliveRequest {
    fn into(self) -> rpc::LeaseKeepAliveRequest {
        let mut req = rpc::LeaseKeepAliveRequest::new();
        req.set_ID(self.id);
        req
    }
}

#[derive(Clone, Debug)]
pub struct KeepAliveResponse {
    header: ResponseHeader,
    id: i64,
    ttl: i64,
}

impl KeepAliveResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ttl(&self) -> i64 {
        self.ttl
    }
}

impl From<rpc::LeaseKeepAliveResponse> for KeepAliveResponse {
    fn from(mut resp: rpc::LeaseKeepAliveResponse) -> Self {
        KeepAliveResponse {
            header: resp.take_header().into(),
            id: resp.ID,
            ttl: resp.TTL,
        }
    }
}

pub struct KeepAlive {
    sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
    receiver: grpcio::ClientDuplexReceiver<rpc::LeaseKeepAliveResponse>,
    req: rpc::LeaseKeepAliveRequest,
    sent: bool,
}

impl KeepAlive {
    pub fn new(
        sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
        receiver: grpcio::ClientDuplexReceiver<rpc::LeaseKeepAliveResponse>,
        req: KeepAliveRequest,
    ) -> Self {
        KeepAlive {
            sender,
            receiver,
            req: req.into(),
            sent: false,
        }
    }
}

impl Stream for KeepAlive {
    type Item = KeepAliveResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if !self.sent {
            match self
                .sender
                .start_send((self.req.clone(), Default::default()))?
            {
                AsyncSink::NotReady(_) => return Ok(Async::NotReady),
                AsyncSink::Ready => {
                    self.sent = true;
                }
            }
        }

        self.sender.poll_complete()?;
        match self.receiver.poll()? {
            Async::Ready(Some(resp)) => {
                self.sent = false;

                Ok(Async::Ready(Some(resp.into())))
            },
            Async::Ready(None) => Ok(Async::Ready(None)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}
