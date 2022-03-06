use crate::lease::LeaseId;
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct LeaseKeepAliveRequest {
    proto: crate::proto::etcdserverpb::LeaseKeepAliveRequest,
}

impl LeaseKeepAliveRequest {
    /// Creates a new LeaseKeepAliveRequest which will refresh the specified lease.
    pub fn new(id: LeaseId) -> Self {
        Self {
            proto: etcdserverpb::LeaseKeepAliveRequest { id: id as i64 },
        }
    }
}

impl From<LeaseKeepAliveRequest> for crate::proto::etcdserverpb::LeaseKeepAliveRequest {
    fn from(x: LeaseKeepAliveRequest) -> Self {
        x.proto
    }
}

#[derive(Debug)]
pub struct LeaseKeepAliveResponse {
    pub header: ResponseHeader,
    pub id: LeaseId,
    pub ttl: u64,
}

impl From<crate::proto::etcdserverpb::LeaseKeepAliveResponse> for LeaseKeepAliveResponse {
    fn from(proto: crate::proto::etcdserverpb::LeaseKeepAliveResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            id: proto.id,
            ttl: proto.ttl as u64,
        }
    }
}
