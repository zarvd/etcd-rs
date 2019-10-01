use crate::proto::lock as rpc;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct LockRequest {
    name: String,
    lease: i64,
}

impl LockRequest {
    pub fn new<N>(name: N, lease_id: i64) -> Self
    where
        N: Into<String>,
    {
        Self {
            name: name.into(),
            lease: lease_id,
        }
    }
}

impl From<LockRequest> for rpc::LockRequest {
    fn from(req: LockRequest) -> Self {
        let mut result = Self::new();
        result.set_name(req.name.into_bytes());
        result.set_lease(req.lease);
        result
    }
}

#[derive(Debug)]
pub struct LockResponse {
    resp: rpc::LockResponse,
}

impl LockResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        self.resp.get_header().clone().into()
    }

    pub fn key(&self) -> &[u8] {
        self.resp.get_key()
    }
}

impl From<rpc::LockResponse> for LockResponse {
    fn from(resp: rpc::LockResponse) -> Self {
        Self { resp }
    }
}
