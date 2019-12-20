use crate::proto::etcdserverpb;
use crate::ResponseHeader;

/// Request for refreshing lease.
#[derive(Debug)]
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
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }

    /// Gets the lease ID for the refreshed lease.
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
