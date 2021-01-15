use crate::proto::etcdserverpb;
use crate::ResponseHeader;

pbwrap_request!(
    /// Request for revoking lease.
    LeaseRevokeRequest
);

impl LeaseRevokeRequest {
    /// Creates a new LeaseRevokeRequest which will revoke the specified lease.
    pub fn new(id: u64) -> Self {
        Self {
            proto: etcdserverpb::LeaseRevokeRequest { id: id as i64 },
        }
    }
}

pbwrap_response!(LeaseRevokeResponse);

impl LeaseRevokeResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        self.proto.header.take().map(From::from)
    }
}
