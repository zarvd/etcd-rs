use super::{KeyRange, KeyValue};
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct DeleteRequest {
    proto: etcdserverpb::DeleteRangeRequest,
}

impl DeleteRequest {
    /// Creates a new DeleteRequest for the specified key range.
    pub fn new<T>(key_range: T) -> Self
    where
        T: Into<KeyRange>,
    {
        let key_range = key_range.into();
        Self {
            proto: etcdserverpb::DeleteRangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                prev_kv: false,
            },
        }
    }

    /// When set, responds with the key-value pair data before the update from this Delete request.
    pub fn prev_kv(mut self, prev_kv: bool) -> Self {
        self.proto.prev_kv = prev_kv;
        self
    }
}

impl<T> From<T> for DeleteRequest
where
    T: Into<KeyRange>,
{
    fn from(key_range: T) -> Self {
        Self::new(key_range)
    }
}

impl From<DeleteRequest> for etcdserverpb::DeleteRangeRequest {
    fn from(value: DeleteRequest) -> Self {
        value.proto
    }
}

#[derive(Debug, Clone)]
pub struct DeleteResponse {
    pub header: ResponseHeader,
    pub deleted: u64,
    pub prev_kvs: Vec<KeyValue>,
}

impl From<etcdserverpb::DeleteRangeResponse> for DeleteResponse {
    fn from(proto: etcdserverpb::DeleteRangeResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            deleted: proto.deleted as u64,
            prev_kvs: proto.prev_kvs.into_iter().map(From::from).collect(),
        }
    }
}
