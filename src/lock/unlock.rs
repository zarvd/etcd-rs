use crate::proto::lock as rpc;
use crate::ResponseHeader;

pub struct UnlockRequest {
    key: Vec<u8>,
}

impl UnlockRequest {
    pub fn new<K>(key: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        Self { key: key.into() }
    }
}

impl From<UnlockRequest> for rpc::UnlockRequest {
    fn from(req: UnlockRequest) -> Self {
        let mut result = Self::new();
        result.set_key(req.key);
        result
    }
}

pub struct UnlockResponse {
    resp: rpc::UnlockResponse,
}

impl UnlockResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        self.resp.get_header().clone().into()
    }
}

impl From<rpc::UnlockResponse> for UnlockResponse {
    fn from(resp: rpc::UnlockResponse) -> Self {
        Self { resp }
    }
}
