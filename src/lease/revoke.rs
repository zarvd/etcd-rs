use crate::proto::rpc;
use crate::ResponseHeader;

pub struct RevokeRequest {
    id: i64,
}

impl RevokeRequest {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl Into<rpc::LeaseRevokeRequest> for RevokeRequest {
    fn into(self) -> rpc::LeaseRevokeRequest {
        let mut req = rpc::LeaseRevokeRequest::new();
        req.set_ID(self.id);
        req
    }
}

pub struct RevokeResponse {
    resp: rpc::LeaseRevokeResponse,
}

impl RevokeResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::LeaseRevokeResponse> for RevokeResponse {
    fn from(resp: rpc::LeaseRevokeResponse) -> Self {
        Self { resp }
    }
}
