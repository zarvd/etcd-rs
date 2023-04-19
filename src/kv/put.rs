use super::KeyValue;
use crate::lease::LeaseId;
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
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

    /// Sets the lease ID to associate with the key in the key-value store.
    /// A lease value of 0 indicates no lease.
    pub fn lease(mut self, lease: LeaseId) -> Self {
        self.proto.lease = lease;
        self
    }

    /// When set, responds with the key-value pair data before the update from this Put request.
    pub fn prev_kv(mut self, prev_kv: bool) -> Self {
        self.proto.prev_kv = prev_kv;
        self
    }

    /// When set, update the key without changing its current value. Returns an error if the key does not exist.
    pub fn ignore_value(mut self) -> Self {
        self.proto.ignore_value = true;
        self
    }

    /// When set, update the key without changing its current lease. Returns an error if the key does not exist.
    pub fn ignore_lease(mut self) -> Self {
        self.proto.ignore_lease = true;
        self
    }
}

impl From<PutRequest> for etcdserverpb::PutRequest {
    fn from(x: PutRequest) -> Self {
        x.proto
    }
}

impl<K, V> From<(K, V)> for PutRequest
where
    K: Into<Vec<u8>>,
    V: Into<Vec<u8>>,
{
    fn from(kv: (K, V)) -> Self {
        Self::new(kv.0, kv.1)
    }
}

#[derive(Debug, Clone)]
pub struct PutResponse {
    pub header: ResponseHeader,
    pub prev_kv: KeyValue,
}

impl From<etcdserverpb::PutResponse> for PutResponse {
    fn from(proto: etcdserverpb::PutResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            prev_kv: From::from(proto.prev_kv.unwrap_or(Default::default())),
        }
    }
}
