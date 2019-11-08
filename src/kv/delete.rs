use super::{KeyValue, KeyRange};
use crate::proto::etcdserverpb;

pub struct DeleteRequest {
    proto: etcdserverpb::DeleteRangeRequest,
}

impl DeleteRequest {
    pub fn new(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::DeleteRangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                prev_kv: false,
            },
        }
    }

    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }
}

impl Into<etcdserverpb::DeleteRangeRequest> for DeleteRequest {
    fn into(self) -> etcdserverpb::DeleteRangeRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct DeleteResponse {
    proto: etcdserverpb::DeleteRangeResponse,
}

impl DeleteResponse {
    pub fn count_deleted(&self) -> usize {
        self.proto.deleted as usize
    }

    pub fn take_prev_kvs(&mut self) -> Vec<KeyValue> {
        let kvs = std::mem::replace(&mut self.proto.prev_kvs, vec![]);

        kvs.into_iter().map(From::from).collect()
    }

    pub fn has_prev_kvs(&self) -> bool {
        !self.proto.prev_kvs.is_empty()
    }
}

impl From<etcdserverpb::DeleteRangeResponse> for DeleteResponse {
    fn from(resp: etcdserverpb::DeleteRangeResponse) -> Self {
        Self { proto: resp }
    }
}
