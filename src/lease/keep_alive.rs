use crate::proto::etcdserverpb;

/// Request for refreshing lease.
pub struct LeaseKeepAliveRequest {
    proto: etcdserverpb::LeaseKeepAliveRequest,
}

impl LeaseKeepAliveRequest {
    /// Creates a new LeaseKeepAliveRequest which will refresh the specified lease.
    pub fn new(id: u64) -> Self {
        let proto = etcdserverpb::LeaseKeepAliveRequest { id: id as i64 };

        Self { proto }
    }
}

impl Into<etcdserverpb::LeaseKeepAliveRequest> for LeaseKeepAliveRequest {
    fn into(self) -> etcdserverpb::LeaseKeepAliveRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct LeaseKeepAliveResponse {
    proto: etcdserverpb::LeaseKeepAliveResponse,
}

impl LeaseKeepAliveResponse {
    /// Get the refreshed lease ID.
    pub fn id(&self) -> u64 {
        self.proto.id as u64
    }

    /// Get the new TTL for the lease.
    pub fn ttl(&self) -> u64 {
        self.proto.ttl as u64
    }
}

impl From<etcdserverpb::LeaseKeepAliveResponse> for LeaseKeepAliveResponse {
    fn from(resp: etcdserverpb::LeaseKeepAliveResponse) -> Self {
        Self { proto: resp }
    }
}
