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
        UnlockRequest { key: key.into() }
    }
}

impl Into<rpc::UnlockRequest> for UnlockRequest {
    fn into(self) -> rpc::UnlockRequest {
        let mut result = rpc::UnlockRequest::new();
        result.set_key(self.key);
        result
    }
}

pub struct UnlockResponse {
    header: ResponseHeader,
}

impl UnlockResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }
}

impl From<rpc::UnlockResponse> for UnlockResponse {
    fn from(mut resp: rpc::UnlockResponse) -> Self {
        UnlockResponse {
            header: resp.take_header().into(),
        }
    }
}
