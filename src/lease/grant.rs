use crate::proto::rpc;
use crate::ResponseHeader;

pub struct GrantRequest {
    ttl: i64,
}

impl GrantRequest {
    pub fn new(ttl: i64) -> Self {
        Self { ttl }
    }
}

impl Into<rpc::LeaseGrantRequest> for GrantRequest {
    fn into(self) -> rpc::LeaseGrantRequest {
        let mut req = rpc::LeaseGrantRequest::new();
        req.set_TTL(self.ttl);
        req
    }
}

pub struct GrantResponse {
    resp: rpc::LeaseGrantResponse,
}

impl GrantResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }

    pub fn id(&self) -> i64 {
        self.resp.get_ID()
    }

    pub fn ttl(&self) -> i64 {
        self.resp.get_TTL()
    }

    pub fn error(&self) -> String {
        // FIXME perf
        self.resp.get_error().to_owned()
    }
}

impl From<rpc::LeaseGrantResponse> for GrantResponse {
    fn from(resp: rpc::LeaseGrantResponse) -> Self {
        Self { resp }
    }
}
