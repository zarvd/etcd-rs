use super::KeyValue;
use crate::proto::etcdserverpb;

pub struct DeleteRequest {
    pub(crate) proto: etcdserverpb::DeleteRangeRequest,
}

impl DeleteRequest {
    fn new(key: Vec<u8>, range_end: Vec<u8>) -> Self {
        Self {
            proto: etcdserverpb::DeleteRangeRequest {
                key,
                range_end,
                prev_kv: false,
            },
        }
    }

    pub fn key<K>(key: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        Self::new(key.into(), vec![])
    }

    pub fn prefix<K>(prefix: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        let key = prefix.into();
        let range_end = {
            let mut end = key.clone();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end = end[0..=i].to_vec();
                    break;
                }
            }
            end
        };
        Self::new(key, range_end)
    }

    pub fn all() -> Self {
        Self::new(vec![0], vec![0])
    }

    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
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
        let kvs = std::mem::take(&mut self.proto.prev_kvs);

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
