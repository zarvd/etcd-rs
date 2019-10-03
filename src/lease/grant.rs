use crate::proto::rpc;
use crate::ResponseHeader;

pub struct GrantRequest {
    ttl: i64,
    id: i64,
}

impl GrantRequest {
    pub fn new(ttl: i64) -> Self {
        GrantRequest { ttl, id: 0 }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }
}

impl Into<rpc::LeaseGrantRequest> for GrantRequest {
    fn into(self) -> rpc::LeaseGrantRequest {
        let mut req = rpc::LeaseGrantRequest::new();

        req.set_TTL(self.ttl);
        req.set_ID(self.id);

        req
    }
}

pub struct GrantResponse {
    header: ResponseHeader,
    id: i64,
    ttl: i64,
    error: String,
}

impl GrantResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ttl(&self) -> i64 {
        self.ttl
    }

    pub fn error(&self) -> &str {
        &self.error
    }
}

impl From<rpc::LeaseGrantResponse> for GrantResponse {
    fn from(mut resp: rpc::LeaseGrantResponse) -> Self {
        GrantResponse {
            header: resp.take_header().into(),
            id: resp.ID,
            ttl: resp.TTL,
            error: resp.error,
        }
    }
}
