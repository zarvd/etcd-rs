use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::watch_request::RequestUnion;
use crate::Event;
use crate::KeyRange;
use crate::ResponseHeader;

pbwrap_request!(
    /// Request for creating or canceling watch.
    #[derive(Debug)]
    WatchRequest
);

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

    /// Sets the revision to watch from (inclusive). No start_revision is "now".
    /// It only effects when the request is for subscribing.
    pub fn set_start_revision(&mut self, revision: usize) {
        // TODO log warning if not CreateRequest
        if let Some(RequestUnion::CreateRequest(ref mut req)) = self.proto.request_union.as_mut() {
            req.start_revision = revision as i64
        }
    }

    /// Sets progress notify.
    /// It only effects when the request is for subscribing.
    pub fn set_progress_notify(&mut self, progress_notify: bool) {
        // TODO log warning if not CreateRequest
        if let Some(RequestUnion::CreateRequest(ref mut req)) = self.proto.request_union.as_mut() {
            req.progress_notify = progress_notify
        }
    }

    /// Sets previous key value.
    /// It only effects when the request is for subscribing.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        // TODO log warning if not CreateRequest
        if let Some(RequestUnion::CreateRequest(ref mut req)) = self.proto.request_union.as_mut() {
            req.prev_kv = prev_kv
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
