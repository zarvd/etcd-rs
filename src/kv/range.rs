use super::{KeyRange, KeyValue};
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

/// Request for fetching key-value pairs.
pub struct RangeRequest {
    proto: etcdserverpb::RangeRequest,
}

impl RangeRequest {
    /// Creates a new RangeRequest for the specified key range.
    pub fn new(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::RangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                limit: 0,
                revision: 0,
                sort_order: 0,
                sort_target: 0,
                serializable: false,
                keys_only: false,
                count_only: false,
                min_mod_revision: 0,
                max_mod_revision: 0,
                min_create_revision: 0,
                max_create_revision: 0,
            },
        }
    }

    /// Sets the maximum number of keys returned for the request.
    /// When limit is set to 0, it is treated as no limit.
    pub fn set_limit(&mut self, limit: usize) {
        self.proto.limit = limit as i64;
    }
}

impl Into<etcdserverpb::RangeRequest> for RangeRequest {
    fn into(self) -> etcdserverpb::RangeRequest {
        self.proto
    }
}

/// Response for RangeRequest.
#[derive(Debug)]
pub struct RangeResponse {
    proto: etcdserverpb::RangeResponse,
}

impl RangeResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }

    /// Takes the key-value pairs out of response, leaving an empty vector in its place.
    pub fn take_kvs(&mut self) -> Vec<KeyValue> {
        let kvs = std::mem::replace(&mut self.proto.kvs, vec![]);

        kvs.into_iter().map(From::from).collect()
    }

    /// Returns `true` if there are more keys to return in the requested range, and `false` otherwise.
    pub fn has_more(&self) -> bool {
        self.proto.more
    }

    /// Returns the number of keys within the range when requested.
    pub fn count(&self) -> usize {
        self.proto.count as usize
    }
}

impl From<etcdserverpb::RangeResponse> for RangeResponse {
    fn from(resp: etcdserverpb::RangeResponse) -> Self {
        Self { proto: resp }
    }
}
