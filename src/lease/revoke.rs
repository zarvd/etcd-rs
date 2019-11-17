use crate::proto::etcdserverpb;
use crate::ResponseHeader;

/// Request for revoking lease.
pub struct LeaseRevokeRequest {
    proto: etcdserverpb::LeaseRevokeRequest,
}

impl LeaseRevokeRequest {
    /// Creates a new LeaseRevokeRequest which will revoke the specified lease.
    pub fn new(id: u64) -> Self {
        let proto = etcdserverpb::LeaseRevokeRequest { id: id as i64 };

        Self { proto }
    }
}

impl Into<etcdserverpb::LeaseRevokeRequest> for LeaseRevokeRequest {
    fn into(self) -> etcdserverpb::LeaseRevokeRequest {
        self.proto
    }
}

/// Response for revoking lease.
#[derive(Debug)]
pub struct LeaseRevokeResponse {
    proto: etcdserverpb::LeaseRevokeResponse,
}

impl LeaseRevokeResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }
}

impl From<etcdserverpb::LeaseRevokeResponse> for LeaseRevokeResponse {
    fn from(resp: etcdserverpb::LeaseRevokeResponse) -> Self {
        Self { proto: resp }
    }
}
