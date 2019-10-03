use std::str::Utf8Error;
use std::string::FromUtf8Error;

use crate::proto::lock as rpc;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct LockRequest {
    name: Vec<u8>,
    lease: i64,
}

impl LockRequest {
    pub fn new<N>(name: N, lease_id: i64) -> Self
    where
        N: Into<Vec<u8>>,
    {
        LockRequest {
            name: name.into(),
            lease: lease_id,
        }
    }
}

impl Into<rpc::LockRequest> for LockRequest {
    fn into(self) -> rpc::LockRequest {
        let mut result = rpc::LockRequest::new();

        result.set_name(self.name);
        result.set_lease(self.lease);

        result
    }
}

#[derive(Clone, Debug)]
pub struct LockResponse {
    header: ResponseHeader,
    key: Vec<u8>,
}

impl LockResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn key(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.key)
    }

    pub fn raw_key(&self) -> &[u8] {
        &self.key
    }

    pub fn into_key(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.key)
    }

    pub fn into_raw_key(self) -> Vec<u8> {
        self.key
    }
}

impl From<rpc::LockResponse> for LockResponse {
    fn from(mut resp: rpc::LockResponse) -> Self {
        LockResponse {
            header: resp.take_header().into(),
            key: resp.key,
        }
    }
}
