use crate::proto::etcdserverpb;
use crate::ResponseHeader;

pbwrap_request!(
    /// Request for refreshing lease.
    #[derive(Debug)]
    LeaseKeepAliveRequest
);

impl LeaseKeepAliveRequest {
    /// Creates a new LeaseKeepAliveRequest which will refresh the specified lease.
    pub fn new(id: u64) -> Self {
        Self {
            proto: etcdserverpb::LeaseKeepAliveRequest { id: id as i64 },
        }
    }
}

pbwrap_response!(LeaseKeepAliveResponse);

impl LeaseKeepAliveResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        self.proto.header.take().map(From::from)
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
