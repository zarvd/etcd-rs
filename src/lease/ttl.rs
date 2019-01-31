use crate::proto::rpc;
use crate::ResponseHeader;

pub struct TtlRequest {
    id: i64,
    with_keys: bool,
}

impl TtlRequest {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            with_keys: false,
        }
    }

    pub fn with_keys(mut self) -> Self {
        self.with_keys = true;
        self
    }
}

impl Into<rpc::LeaseTimeToLiveRequest> for TtlRequest {
    fn into(self) -> rpc::LeaseTimeToLiveRequest {
        let mut req = rpc::LeaseTimeToLiveRequest::new();
        req.set_ID(self.id);
        req.set_keys(self.with_keys);
        req
    }
}

pub struct TtlResponse {
    resp: rpc::LeaseTimeToLiveResponse,
}

impl TtlResponse {
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

    pub fn granted_ttl(&self) -> i64 {
        self.resp.get_grantedTTL()
    }

    pub fn keys(&self) -> String {
        // FIXME perf
        self.resp
            .get_keys()
            .iter()
            .map(|key| std::str::from_utf8(key).unwrap())
            .collect()
    }
}

impl From<rpc::LeaseTimeToLiveResponse> for TtlResponse {
    fn from(resp: rpc::LeaseTimeToLiveResponse) -> Self {
        Self { resp }
    }
}
