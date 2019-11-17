use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::watch_request::RequestUnion;
use crate::KeyRange;

/// Request for creating or canceling watch.
pub struct WatchRequest {
    proto: etcdserverpb::WatchRequest,
}

impl WatchRequest {
    /// Creates a new WatchRequest which will subscribe events of the specified key.
    pub fn create(mut key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::WatchRequest {
                request_union: Some(RequestUnion::CreateRequest(
                    etcdserverpb::WatchCreateRequest {
                        key: key_range.take_key(),
                        range_end: key_range.take_range_end(),
                        start_revision: 0,
                        progress_notify: false,
                        filters: vec![], // TODO support filters
                        prev_kv: false,
                    },
                )),
            },
        }
    }

    /// Creates a new WatchRequest which will unsubscribe the specified watch.
    pub fn cancel(watch_id: usize) -> Self {
        Self {
            proto: etcdserverpb::WatchRequest {
                request_union: Some(RequestUnion::CancelRequest(
                    etcdserverpb::WatchCancelRequest {
                        watch_id: watch_id as i64,
                    },
                )),
            },
        }
    }

    /// Set start revision.
    /// It only effects when the request is for subscribing.
    pub fn set_start_revision(&mut self, revision: usize) {
        // TODO log warning if not CreateRequest
        match self.proto.request_union.as_mut().unwrap() {
            RequestUnion::CreateRequest(ref mut req) => req.start_revision = revision as i64,
            _ => {}
        }
    }

    /// Set progress notify.
    /// It only effects when the request is for subscribing.
    pub fn set_progress_notify(&mut self, progress_notify: bool) {
        // TODO log warning if not CreateRequest
        match self.proto.request_union.as_mut().unwrap() {
            RequestUnion::CreateRequest(ref mut req) => req.progress_notify = progress_notify,
            _ => {}
        }
    }

    /// Set previous key value.
    /// It only effects when the request is for subscribing.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        // TODO log warning if not CreateRequest
        match self.proto.request_union.as_mut().unwrap() {
            RequestUnion::CreateRequest(ref mut req) => req.prev_kv = prev_kv,
            _ => {}
        }
    }
}

impl Into<etcdserverpb::WatchRequest> for WatchRequest {
    fn into(self) -> etcdserverpb::WatchRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct WatchResponse {
    proto: etcdserverpb::WatchResponse,
}

impl WatchResponse {}

impl From<etcdserverpb::WatchResponse> for WatchResponse {
    fn from(resp: etcdserverpb::WatchResponse) -> Self {
        Self { proto: resp }
    }
}
