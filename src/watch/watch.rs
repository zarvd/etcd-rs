use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::watch_request::RequestUnion;
use crate::{Event, KeyRange, ResponseHeader};

#[derive(Debug)]
pub struct WatchCreateRequest {
    proto: crate::proto::etcdserverpb::WatchCreateRequest,
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
    pub fn start_revision(mut self, revision: i64) -> Self {
        self.proto.start_revision = revision;
        self
    }

    pub fn progress_notify(mut self) -> Self {
        self.proto.progress_notify = true;
        self
    }

    /// Sets previous key value.
    pub fn prev_kv(mut self) -> Self {
        self.proto.prev_kv = true;
        self
    }
}
impl Into<etcdserverpb::WatchCreateRequest> for WatchCreateRequest {
    fn into(self) -> etcdserverpb::WatchCreateRequest {
        self.proto
    }
}

impl Into<etcdserverpb::WatchRequest> for WatchCreateRequest {
    fn into(self) -> etcdserverpb::WatchRequest {
        etcdserverpb::WatchRequest {
            request_union: Some(RequestUnion::CreateRequest(self.into())),
        }
    }
}

impl From<KeyRange> for WatchCreateRequest {
    fn from(key_range: KeyRange) -> Self {
        Self::create(key_range)
    }
}

#[derive(Debug, Clone)]
pub struct WatchCancelRequest {
    proto: etcdserverpb::WatchCancelRequest,
}

impl WatchCancelRequest {
    /// Creates a new WatchRequest which will unsubscribe the specified watch.
    pub fn new(watch_id: i64) -> Self {
        Self {
            proto: etcdserverpb::WatchCancelRequest { watch_id },
        }
    }
}

impl From<i64> for WatchCancelRequest {
    fn from(watch_id: i64) -> Self {
        Self::new(watch_id)
    }
}

impl Into<etcdserverpb::WatchCancelRequest> for WatchCancelRequest {
    fn into(self) -> etcdserverpb::WatchCancelRequest {
        self.proto
    }
}

impl Into<etcdserverpb::WatchRequest> for WatchCancelRequest {
    fn into(self) -> etcdserverpb::WatchRequest {
        etcdserverpb::WatchRequest {
            request_union: Some(RequestUnion::CancelRequest(self.into())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WatchResponse {
    pub header: ResponseHeader,
    pub watch_id: i64,
    pub created: bool,
    pub canceled: bool,
    pub events: Vec<Event>,
}

impl From<etcdserverpb::WatchResponse> for WatchResponse {
    fn from(proto: etcdserverpb::WatchResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            watch_id: proto.watch_id,
            created: proto.created,
            canceled: proto.canceled,
            events: proto.events.into_iter().map(From::from).collect(),
        }
    }
}
