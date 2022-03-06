use crate::lease::LeaseId;
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct LeaseRevokeRequest {
    proto: crate::proto::etcdserverpb::LeaseRevokeRequest,
}

impl LeaseRevokeRequest {
    /// Creates a new LeaseRevokeRequest which will revoke the specified lease.
    pub fn new(id: LeaseId) -> Self {
        Self {
            proto: etcdserverpb::LeaseRevokeRequest { id: id as LeaseId },
        }
    }
}
impl From<LeaseRevokeRequest> for crate::proto::etcdserverpb::LeaseRevokeRequest {
    fn from(x: LeaseRevokeRequest) -> Self {
        x.proto
    }
}

#[derive(Debug, Clone)]
pub struct LeaseRevokeResponse {
    pub header: ResponseHeader,
}

impl From<crate::proto::etcdserverpb::LeaseRevokeResponse> for LeaseRevokeResponse {
    fn from(proto: crate::proto::etcdserverpb::LeaseRevokeResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
