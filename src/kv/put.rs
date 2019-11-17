use crate::proto::etcdserverpb;

/// Request for putting key value.
pub struct PutRequest {
    proto: etcdserverpb::PutRequest,
}

impl PutRequest {
    /// Creates a new PutRequest for saving the specified key-value.
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

    /// Set custome lease.
    pub fn set_lease(&mut self, lease: u64) {
        self.proto.lease = lease as i64;
    }

    /// Set previous key-value.
    /// When set, responds with the key-value pair data before the update from this Put request.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }

    /// Set ignore value.
    /// When set, update the key without changing its current value. Returns an error if the key does not exist.
    pub fn set_ignore_value(&mut self, ignore_value: bool) {
        self.proto.ignore_value = ignore_value;
    }

    /// Set ignore lease.
    /// When set, update the key without changing its current lease. Returns an error if the key does not exist.
    pub fn set_ignore_lease(&mut self, ignore_lease: bool) {
        self.proto.ignore_lease = ignore_lease;
    }
}

impl Into<etcdserverpb::PutRequest> for PutRequest {
    fn into(self) -> etcdserverpb::PutRequest {
        self.proto
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
