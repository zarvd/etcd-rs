use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct RevokeRequest {
    id: i64,
}

impl RevokeRequest {
    pub fn new(id: i64) -> Self {
        RevokeRequest { id }
    }
}

impl Into<rpc::LeaseRevokeRequest> for RevokeRequest {
    fn into(self) -> rpc::LeaseRevokeRequest {
        let mut req = rpc::LeaseRevokeRequest::new();
        req.set_ID(self.id);
        req
    }
}

#[derive(Clone, Debug)]
pub struct RevokeResponse {
    header: ResponseHeader,
}

impl RevokeResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }
}

impl From<rpc::LeaseRevokeResponse> for RevokeResponse {
    fn from(mut resp: rpc::LeaseRevokeResponse) -> Self {
        RevokeResponse {
            header: resp.take_header().into(),
        }
    }
}
