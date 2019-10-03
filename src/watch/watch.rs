use futures::{Async, AsyncSink, Poll, Sink, Stream};

use crate::kv::Event;
use crate::proto::rpc;
use crate::Error;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct WatchRequest {
    key: Vec<u8>,
    end_key: Option<Vec<u8>>,
    start_revision: i64,
    progress_notify: bool,
    filters: Vec<rpc::WatchCreateRequest_FilterType>,
    prev_kv: bool,
    watch_id: i64,
    fragment: bool,
}

impl WatchRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        WatchRequest {
            key: key.into(),
            end_key: None,
            start_revision: 0,
            progress_notify: false,
            filters: Default::default(),
            prev_kv: false,
            watch_id: 0,
            fragment: false,
        }
    }

    pub fn prefix<N>(prefix: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let key = prefix.into();
        let end_key = {
            let mut end = key.clone();
            let last = end.last().copied().unwrap_or(0);

            if last == std::u8::MAX {
                end.push(1);
            } else {
                *end.last_mut().unwrap() += 1;
            }

            end
        };

        WatchRequest {
            key,
            end_key: Some(end_key),
            start_revision: 0,
            progress_notify: false,
            filters: Default::default(),
            prev_kv: false,
            watch_id: 0,
            fragment: false,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        WatchRequest {
            key: key.into(),
            end_key: Some(end_key.into()),
            start_revision: 0,
            progress_notify: false,
            filters: Default::default(),
            prev_kv: false,
            watch_id: 0,
            fragment: false,
        }
    }

    pub fn with_start_revision(mut self, revision: i64) -> Self {
        self.start_revision = revision;
        self
    }

    pub fn with_progress_notify(mut self) -> Self {
        self.progress_notify = true;
        self
    }

    pub fn with_filter_no_put(mut self) -> Self {
        let filter = rpc::WatchCreateRequest_FilterType::NOPUT;
        if !self.filters.contains(&filter) {
            self.filters.push(filter);
        }

        self
    }

    pub fn with_filter_no_delete(mut self) -> Self {
        let filter = rpc::WatchCreateRequest_FilterType::NODELETE;
        if !self.filters.contains(&filter) {
            self.filters.push(filter);
        }

        self
    }

    pub fn with_prev_kv(mut self) -> Self {
        self.prev_kv = true;
        self
    }

    pub fn with_watch_id(mut self, id: i64) -> Self {
        self.watch_id = id;
        self
    }

    pub fn with_fragment(mut self) -> Self {
        self.fragment = true;
        self
    }
}

impl Into<rpc::WatchCreateRequest> for WatchRequest {
    fn into(self) -> rpc::WatchCreateRequest {
        let mut req = rpc::WatchCreateRequest::new();

        req.set_key(self.key);
        req.set_start_revision(self.start_revision);
        req.set_progress_notify(self.progress_notify);
        req.set_filters(self.filters);
        req.set_prev_kv(self.prev_kv);
        req.set_prev_kv(self.prev_kv);
        req.set_watch_id(self.watch_id);
        req.set_fragment(self.fragment);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }

        req
    }
}

#[derive(Clone, Debug)]
pub struct WatchResponse {
    header: ResponseHeader,
    watch_id: i64,
    created: bool,
    canceled: bool,
    compact_revision: i64,
    cancel_reason: String,
    fragment: bool,
    events: Vec<Event>,
}

impl WatchResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn watch_id(&self) -> i64 {
        self.watch_id
    }

    pub fn is_created(&self) -> bool {
        self.created
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }

    pub fn compact_revision(&self) -> i64 {
        self.compact_revision
    }

    pub fn cancel_reason(&self) -> &str {
        &self.cancel_reason
    }

    pub fn fragment(&self) -> bool {
        self.fragment
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

impl From<rpc::WatchResponse> for WatchResponse {
    fn from(mut resp: rpc::WatchResponse) -> Self {
        WatchResponse {
            header: resp.take_header().into(),
            watch_id: resp.watch_id,
            created: resp.created,
            canceled: resp.canceled,
            compact_revision: resp.compact_revision,
            cancel_reason: resp.cancel_reason,
            fragment: resp.fragment,
            events: resp.events.into_vec().into_iter().map(Into::into).collect(),
        }
    }
}

pub(crate) struct Watch {
    sender: grpcio::ClientDuplexSender<rpc::WatchRequest>,
    receiver: grpcio::ClientDuplexReceiver<rpc::WatchResponse>,
    create_req: rpc::WatchRequest,
    sent: bool,
}

impl Watch {
    pub(crate) fn new(
        sender: grpcio::ClientDuplexSender<rpc::WatchRequest>,
        receiver: grpcio::ClientDuplexReceiver<rpc::WatchResponse>,
        req: WatchRequest,
    ) -> Self {
        let create_req = {
            let mut create_req = rpc::WatchRequest::new();
            create_req.set_create_request(req.into());
            create_req
        };

        Watch {
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
                .start_send((self.create_req.clone(), Default::default()))?
            {
                AsyncSink::NotReady(_) => return Ok(Async::NotReady),
                AsyncSink::Ready => {
                    self.sent = true;
                }
            }
        }

        self.sender.poll_complete()?;
        match self.receiver.poll()? {
            Async::Ready(Some(resp)) => Ok(Async::Ready(Some(resp.into()))),
            Async::Ready(None) => Ok(Async::Ready(None)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}
