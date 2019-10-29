use crate::proto::etcdserverpb;

pub struct PutRequest {
    pub(crate) proto: etcdserverpb::PutRequest,
}

impl PutRequest {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        Self {
            proto: etcdserverpb::PutRequest {
                key: key.into(),
                value: value.into(),
                lease: 0,
                prev_kv: false,
                ignore_value: false,
                ignore_lease: false,
            },
        }
    }

    pub fn set_lease(&mut self, lease: i64) {
        self.proto.lease = lease;
    }

    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }

    pub fn set_ignore_value(&mut self, ignore_value: bool) {
        self.proto.ignore_value = ignore_value;
    }

    pub fn set_ignore_lease(&mut self, ignore_lease: bool) {
        self.proto.ignore_lease = ignore_lease;
    }
}

#[derive(Debug)]
pub struct PutResponse {
    proto: etcdserverpb::PutResponse,
}

impl From<etcdserverpb::PutResponse> for PutResponse {
    fn from(resp: etcdserverpb::PutResponse) -> Self {
        Self { proto: resp }
    }
}
