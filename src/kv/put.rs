use crate::proto::etcdserverpb;
use crate::KeyValue;
use crate::ResponseHeader;

pbwrap_request!(
/// Request for putting key-value.
PutRequest
);

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
    pub fn set_lease(&mut self, lease: u64) {
        self.proto.lease = lease as i64;
    }

    /// When set, responds with the key-value pair data before the update from this Put request.
    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }

    /// When set, update the key without changing its current value. Returns an error if the key does not exist.
    pub fn set_ignore_value(&mut self, ignore_value: bool) {
        self.proto.ignore_value = ignore_value;
    }

    /// When set, update the key without changing its current lease. Returns an error if the key does not exist.
    pub fn set_ignore_lease(&mut self, ignore_lease: bool) {
        self.proto.ignore_lease = ignore_lease;
    }
}

pbwrap_response!(PutResponse);

impl PutResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }

    /// Takes the previous key-value pair out of response, leaving a `None` in its place.
    pub fn take_prev_kv(&mut self) -> Option<KeyValue> {
        match self.proto.prev_kv.take() {
            Some(kv) => Some(From::from(kv)),
            _ => None,
        }
    }
}
