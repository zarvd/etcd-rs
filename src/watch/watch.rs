use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::watch_request::RequestUnion;
use crate::Event;
use crate::KeyRange;
use crate::ResponseHeader;

pbwrap_request!(
    /// Request for creating watch.
    #[derive(Debug, Clone)]
    WatchCreateRequest
);

impl From<WatchCreateRequest> for etcdserverpb::WatchRequest {
    fn from(x: WatchCreateRequest) -> Self {
        etcdserverpb::WatchRequest {
            request_union: Some(RequestUnion::CreateRequest(x.into())),
        }
    }
}

impl From<KeyRange> for WatchCreateRequest {
    fn from(key_range: KeyRange) -> Self {
        Self::create(key_range)
    }
}

impl WatchCreateRequest {
    /// Creates a new WatchRequest which will subscribe events of the specified key.
    pub fn create(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::WatchCreateRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                start_revision: 0,
                progress_notify: false,
                filters: vec![], // TODO support filters
                prev_kv: false,
            },
        }
    }

    /// Sets the revision to watch from (inclusive). No start_revision is "now".
    pub fn set_start_revision(&mut self, revision: u64) {
        self.proto.start_revision = revision as i64;
    }

    pub fn set_progress_notify(&mut self, progress_notify: bool) {
        self.proto.progress_notify = progress_notify;
    }

    /// Sets previous key value.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }
}

pbwrap_request!(
    /// Request for canceling a watch.
    #[derive(Debug)]
    WatchCancelRequest
);

impl From<WatchCancelRequest> for etcdserverpb::WatchRequest {
    fn from(x: WatchCancelRequest) -> Self {
        etcdserverpb::WatchRequest {
            request_union: Some(RequestUnion::CancelRequest(x.into())),
        }
    }
}

impl WatchCancelRequest {
    /// Creates a new WatchRequest which will unsubscribe the specified watch.
    pub fn cancel(watch_id: usize) -> Self {
        Self {
            proto: etcdserverpb::WatchCancelRequest {
                watch_id: watch_id as i64,
            },
        }
    }
}

pbwrap_response!(WatchResponse);

impl WatchResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }

    /// Gets the ID of the watcher that corresponds to the response.
    pub fn watch_id(&self) -> u64 {
        self.proto.watch_id as u64
    }

    /// Takes the events out of response, leaving an empty vector in its place.
    pub fn take_events(&mut self) -> Vec<Event> {
        let events = std::mem::take(&mut self.proto.events);

        events.into_iter().map(From::from).collect()
    }
}
