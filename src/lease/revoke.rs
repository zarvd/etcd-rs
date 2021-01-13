use crate::proto::etcdserverpb;
use crate::ResponseHeader;

pbwrap_request!(
/// Request for revoking lease.
LeaseRevokeRequest
);

impl LeaseRevokeRequest {
    /// Creates a new LeaseRevokeRequest which will revoke the specified lease.
    pub fn new(id: u64) -> Self {
        let proto = etcdserverpb::LeaseRevokeRequest { id: id as i64 };

        Self { proto }
    }
}

pbwrap_response!(LeaseRevokeResponse);

impl LeaseRevokeResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }
}
