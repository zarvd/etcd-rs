mod delete;
mod put;
mod range;
mod txn;

pub use delete::{DeleteRequest, DeleteResponse};
pub use put::{PutRequest, PutResponse};
pub use range::{RangeRequest, RangeResponse};
pub use txn::{TxnCmp, TxnRequest, TxnResponse};

use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::KvClient;
use crate::proto::{etcdserverpb, mvccpb};
use crate::Result;

#[derive(Clone)]
pub struct Kv {
    client: KvClient<Channel>,
}

/// Key-Value client
impl Kv {
    pub(crate) fn new(client: KvClient<Channel>) -> Self {
        Self { client }
    }

    pub async fn put(&mut self, req: PutRequest) -> Result<PutResponse> {
        let resp = self.client.put(tonic::Request::new(req.proto)).await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn range(&mut self, req: RangeRequest) -> Result<RangeResponse> {
        let resp = self.client.range(tonic::Request::new(req.proto)).await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn delete(&mut self, req: DeleteRequest) -> Result<DeleteResponse> {
        let resp = self
            .client
            .delete_range(tonic::Request::new(req.proto))
            .await?;

        Ok(From::from(resp.into_inner()))
    }
}

/// Key-Value pair
#[derive(Clone, PartialEq)]
pub struct KeyValue {
    proto: mvccpb::KeyValue,
}

impl KeyValue {
    pub fn key(&self) -> &[u8] {
        &self.proto.key
    }

    pub fn take_key(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.proto.key)
    }

    pub fn key_str(&self) -> &str {
        std::str::from_utf8(&self.proto.key).expect("convert bytes to string")
    }

    pub fn value(&self) -> &[u8] {
        &self.proto.value
    }

    pub fn take_value(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.proto.value)
    }

    pub fn value_str(&self) -> &str {
        std::str::from_utf8(&self.proto.value).expect("convert bytes to string")
    }

    pub fn create_revision(&self) -> usize {
        self.proto.create_revision as usize
    }

    pub fn mod_revision(&self) -> usize {
        self.proto.mod_revision as usize
    }

    pub fn version(&self) -> usize {
        self.proto.version as usize
    }

    pub fn lease(&self) -> usize {
        self.proto.lease as usize
    }

    pub fn has_lease(&self) -> bool {
        self.proto.lease != 0
    }
}

impl From<mvccpb::KeyValue> for KeyValue {
    fn from(kv: mvccpb::KeyValue) -> Self {
        Self { proto: kv }
    }
}
