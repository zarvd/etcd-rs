use super::{KeyRange, KeyValue};
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

pbwrap_request!(
    /// Request for deleting key-value pairs.
    DeleteRangeRequest => DeleteRequest
);

impl DeleteRequest {
    /// Creates a new DeleteRequest for the specified key range.
    pub fn new(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::DeleteRangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                prev_kv: false,
            },
        }
    }

    /// When set, responds with the key-value pair data before the update from this Delete request.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }
}

pbwrap_response!(DeleteRangeResponse => DeleteResponse);

impl DeleteResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        self.proto.header.take().map(From::from)
    }

    /// Returns the number of keys deleted by the delete range request.
    pub fn count_deleted(&self) -> usize {
        self.proto.deleted as usize
    }

    /// Takes the previous key-value pairs out of response, leaving an empty vector in its place.
    pub fn take_prev_kvs(&mut self) -> Vec<KeyValue> {
        std::mem::take(&mut self.proto.prev_kvs)
            .into_iter()
            .map(From::from)
            .collect()
    }

    /// Returns `true` if the previous key-value pairs is not empty, and `false` otherwise.
    pub fn has_prev_kvs(&self) -> bool {
        !self.proto.prev_kvs.is_empty()
    }
}
