use std::sync::Arc;
use tonic::transport::Channel;

use crate::client::Inner;
use crate::proto::etcdserverpb as pb;
use crate::proto::etcdserverpb::client::KvClient;

#[derive(Clone)]
pub struct Kv {
    client: KvClient<Channel>,
}

impl Kv {
    pub(crate) fn new(client: KvClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn put(
        &mut self,
        req: PutRequest,
    ) -> Result<PutResponse, Box<dyn std::error::Error>> {
        let resp = self.client.put(tonic::Request::new(req.proto)).await?;

        Ok(PutResponse::from(resp.into_inner()))
    }
}

pub struct PutRequest {
    proto: pb::PutRequest,
}

impl PutRequest {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        Self {
            proto: pb::PutRequest {
                key: key.into(),
                value: value.into(),
                lease: 0,
                prev_kv: false,
                ignore_value: false,
                ignore_lease: false,
            },
        }
    }

    pub fn set_lease(&mut self, lease: i64) {
        self.proto.lease = lease;
    }

    pub fn set_prev_kv(&mut self, prev_kv: bool) {
        self.proto.prev_kv = prev_kv;
    }

    pub fn set_ignore_value(&mut self, ignore_value: bool) {
        self.proto.ignore_value = ignore_value;
    }

    pub fn set_ignore_lease(&mut self, ignore_lease: bool) {
        self.proto.ignore_lease = ignore_lease;
    }
}

#[derive(Debug)]
pub struct PutResponse {
    proto: pb::PutResponse,
}

impl From<pb::PutResponse> for PutResponse {
    fn from(resp: pb::PutResponse) -> Self {
        Self { proto: resp }
    }
}
