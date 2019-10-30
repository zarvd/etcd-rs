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
use crate::proto::mvccpb;
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
        let resp = self.client.put(tonic::Request::new(req.into())).await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn range(&mut self, req: RangeRequest) -> Result<RangeResponse> {
        let resp = self.client.range(tonic::Request::new(req.into())).await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn delete(&mut self, req: DeleteRequest) -> Result<DeleteResponse> {
        let resp = self
            .client
            .delete_range(tonic::Request::new(req.into()))
            .await?;

        Ok(From::from(resp.into_inner()))
    }

    pub async fn txn(&mut self, req: TxnRequest) -> Result<TxnResponse> {
        let resp = self.client.txn(tonic::Request::new(req.into())).await?;

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

/// KeyRange
pub struct KeyRange {
    key: Vec<u8>,
    range_end: Vec<u8>,
}

impl KeyRange {
    pub fn range<K, R>(key: K, range_end: R) -> Self
    where
        K: Into<Vec<u8>>,
        R: Into<Vec<u8>>,
    {
        Self {
            key: key.into(),
            range_end: range_end.into(),
        }
    }

    pub fn key<K>(key: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        Self {
            key: key.into(),
            range_end: vec![],
        }
    }

    pub fn all() -> Self {
        Self {
            key: vec![0],
            range_end: vec![0],
        }
    }

    pub fn prefix<K>(prefix: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        let key = prefix.into();
        let range_end = {
            let mut end = key.clone();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end = end[0..=i].to_vec();
                    break;
                }
            }
            end
        };
        Self { key, range_end }
    }

    pub fn take_key(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.key)
    }

    pub fn take_range_end(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.range_end)
    }
}
