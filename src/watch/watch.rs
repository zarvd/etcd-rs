use futures::{Async, AsyncSink, Poll, Sink, Stream};

use crate::kv::Event;
use crate::proto::rpc;
use crate::Error;
use crate::ResponseHeader;

pub struct WatchRequest {
    key: String,
    end_key: Option<Vec<u8>>,
    start_revision: i64,
    progress_notify: bool,
    prev_kv: bool,
}

impl WatchRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            end_key: None,
            start_revision: 0,
            progress_notify: false,
            prev_kv: false,
        }
    }

    pub fn prefix<N>(prefix: N) -> Self
    where
        N: Into<String>,
    {
        let key = prefix.into();
        let end_key = {
            let mut end = key.clone().into_bytes();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end = end[0..i + 1].to_vec();
                    break;
                }
            }

            end
        };
        Self {
            key: key,
            end_key: Some(end_key),
            start_revision: 0,
            progress_notify: false,
            prev_kv: false,
        }
    }

    pub fn with_prev_kv(mut self) -> Self {
        self.prev_kv = true;
        self
    }

    pub fn with_progress_notify(mut self) -> Self {
        self.progress_notify = true;
        self
    }

    pub fn with_start_revision(mut self, revision: i64) -> Self {
        self.start_revision = revision;
        self
    }
}

impl Into<rpc::WatchCreateRequest> for WatchRequest {
    fn into(self) -> rpc::WatchCreateRequest {
        let mut req = rpc::WatchCreateRequest::new();
        req.set_key(self.key.into_bytes());
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }
        req.set_start_revision(self.start_revision);
        req.set_prev_kv(self.prev_kv);
        req.set_progress_notify(self.progress_notify);

        req
    }
}

#[derive(Debug)]
pub struct WatchResponse {
    resp: rpc::WatchResponse,
}

impl WatchResponse {
    pub fn watch_id(&self) -> i64 {
        self.resp.get_watch_id()
    }

    pub fn is_created(&self) -> bool {
        self.resp.get_created()
    }

    pub fn is_canceled(&self) -> bool {
        self.resp.get_canceled()
    }

    pub fn compact_revision(&self) -> i64 {
        self.resp.get_compact_revision()
    }

    pub fn events(&self) -> Vec<Event> {
        // FIXME perf
        self.resp
            .get_events()
            .iter()
            .map(|e| From::from(e.clone()))
            .collect()
    }

    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::WatchResponse> for WatchResponse {
    fn from(resp: rpc::WatchResponse) -> Self {
        Self { resp }
    }
}

pub struct Watch {
    sender: grpcio::ClientDuplexSender<rpc::WatchRequest>,
    receiver: grpcio::ClientDuplexReceiver<rpc::WatchResponse>,
    create_req: rpc::WatchRequest,
    sent: bool,
}

impl Watch {
    pub fn new(
        sender: grpcio::ClientDuplexSender<rpc::WatchRequest>,
        receiver: grpcio::ClientDuplexReceiver<rpc::WatchResponse>,
        req: WatchRequest,
    ) -> Self {
        let create_req = {
            let mut create_req = rpc::WatchRequest::new();
            create_req.set_create_request(req.into());
            create_req
        };

        Self {
            sender,
            receiver,
            create_req,
            sent: false,
        }
    }
}

impl Stream for Watch {
    type Item = WatchResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if !self.sent {
            match self
                .sender
                .start_send((self.create_req.clone(), Default::default()))
            {
                Ok(AsyncSink::NotReady(_)) => return Ok(Async::NotReady),
                Ok(AsyncSink::Ready) => {
                    self.sent = true;
                }
                Err(e) => return Err(Error::GrpcFailure(e)),
            }
        }

        self.sender.poll_complete().unwrap();

        match self.receiver.poll() {
            Ok(Async::Ready(Some(resp))) => Ok(Async::Ready(Some(From::from(resp)))),
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(Error::GrpcFailure(e)),
        }
    }
}
