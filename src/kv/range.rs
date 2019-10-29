use super::KeyValue;
use crate::proto::etcdserverpb;

pub struct RangeRequest {
    pub(crate) proto: etcdserverpb::RangeRequest,
}

impl RangeRequest {
    fn new(key: Vec<u8>, range_end: Vec<u8>) -> Self {
        Self {
            proto: etcdserverpb::RangeRequest {
                key: key,
                range_end: range_end,
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

    pub fn all() -> Self {
        Self::new(vec![0], vec![0])
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

    pub fn range<K, V>(key: K, range_end: V) -> Self
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        Self::new(key.into(), range_end.into())
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.proto.limit = limit as i64;
    }
}

#[derive(Debug)]
pub struct RangeResponse {
    proto: etcdserverpb::RangeResponse,
}

impl RangeResponse {
    pub fn take_kvs(&mut self) -> Vec<KeyValue> {
        let kvs = std::mem::take(&mut self.proto.kvs);

        kvs.into_iter().map(From::from).collect()
    }

    pub fn has_more(&self) -> bool {
        self.proto.more
    }

    pub fn count(&self) -> usize {
        self.proto.count as usize
    }
}

impl From<etcdserverpb::RangeResponse> for RangeResponse {
    fn from(resp: etcdserverpb::RangeResponse) -> Self {
        Self { proto: resp }
    }
}
