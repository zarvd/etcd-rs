use std::time::Duration;

use crate::lease::LeaseId;
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct LeaseGrantRequest {
    proto: crate::proto::etcdserverpb::LeaseGrantRequest,
}

impl LeaseGrantRequest {
    /// Creates a new LeaseGrantRequest with the specified TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            proto: etcdserverpb::LeaseGrantRequest {
                ttl: ttl.as_secs() as i64,
                id: 0,
            },
        }
    }

    /// Set custom lease ID.
    pub fn with_id(mut self, id: LeaseId) -> Self {
        self.proto.id = id as LeaseId;
        self
    }
}

impl From<LeaseGrantRequest> for crate::proto::etcdserverpb::LeaseGrantRequest {
    fn from(x: LeaseGrantRequest) -> Self {
        x.proto
    }
}

impl From<Duration> for LeaseGrantRequest {
    fn from(ttl: Duration) -> Self {
        Self::new(ttl)
    }
}

#[derive(Debug, Clone)]
pub struct LeaseGrantResponse {
    pub header: ResponseHeader,
    pub id: LeaseId,
    pub ttl: u64,
}

impl From<crate::proto::etcdserverpb::LeaseGrantResponse> for LeaseGrantResponse {
    fn from(proto: crate::proto::etcdserverpb::LeaseGrantResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            id: proto.id,
            ttl: proto.ttl as u64,
        }
    }
}
