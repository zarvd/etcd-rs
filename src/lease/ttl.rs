use std::str::Utf8Error;

use crate::proto::rpc;
use crate::ResponseHeader;

pub struct TtlRequest {
    id: i64,
    with_keys: bool,
}

impl TtlRequest {
    pub fn new(id: i64) -> Self {
        TtlRequest {
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
    header: ResponseHeader,
    id: i64,
    ttl: i64,
    granted_ttl: i64,
    keys: Vec<Vec<u8>>,
}

impl TtlResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ttl(&self) -> i64 {
        self.ttl
    }

    pub fn granted_ttl(&self) -> i64 {
        self.granted_ttl
    }

    pub fn keys(&self) -> Result<Vec<&str>, Utf8Error> {
        let mut keys = Vec::with_capacity(self.keys.len());
        for key in &self.keys {
            keys.push(std::str::from_utf8(key)?);
        }

        Ok(keys)
    }

    pub fn raw_keys(&self) -> &[Vec<u8>] {
        &self.keys
    }
}

impl From<rpc::LeaseTimeToLiveResponse> for TtlResponse {
    fn from(mut resp: rpc::LeaseTimeToLiveResponse) -> Self {
        TtlResponse {
            header: resp.take_header().into(),
            id: resp.ID,
            ttl: resp.TTL,
            granted_ttl: resp.grantedTTL,
            keys: resp.keys.into_vec(),
        }
    }
}
