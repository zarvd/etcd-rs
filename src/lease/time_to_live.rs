use crate::proto::etcdserverpb;
use crate::{LeaseId, ResponseHeader};

#[derive(Debug)]
pub struct LeaseTimeToLiveRequest {
    proto: crate::proto::etcdserverpb::LeaseTimeToLiveRequest,
}

impl LeaseTimeToLiveRequest {
    /// Creates a new LeaseTimeToLiveRequest with the specified lease id.
    pub fn new(id: LeaseId) -> Self {
        Self {
            proto: etcdserverpb::LeaseTimeToLiveRequest { id, keys: false },
        }
    }

    /// Set custom lease ID.
    pub fn with_id(mut self, id: LeaseId) -> Self {
        self.proto.id = id;
        self
    }

    pub fn with_keys(mut self, keys: bool) -> Self {
        self.proto.keys = keys;
        self
    }
}

impl From<LeaseTimeToLiveRequest> for crate::proto::etcdserverpb::LeaseTimeToLiveRequest {
    fn from(x: LeaseTimeToLiveRequest) -> Self {
        x.proto
    }
}

impl From<LeaseId> for LeaseTimeToLiveRequest {
    fn from(lease_id: LeaseId) -> Self {
        Self::new(lease_id)
    }
}

#[derive(Debug)]
pub struct LeaseTimeToLiveResponse {
    pub header: ResponseHeader,
    pub id: LeaseId,
    pub ttl: i64,
}

impl From<crate::proto::etcdserverpb::LeaseTimeToLiveResponse> for LeaseTimeToLiveResponse {
    fn from(proto: crate::proto::etcdserverpb::LeaseTimeToLiveResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            id: proto.id,
            ttl: proto.ttl,
        }
    }
}
