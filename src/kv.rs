use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::KvClient;
use crate::proto::{etcdserverpb, mvccpb};
use crate::Result;

#[derive(Clone)]
pub struct Kv {
    client: KvClient<Channel>,
}

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
}

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

pub struct PutRequest {
    proto: etcdserverpb::PutRequest,
}

impl PutRequest {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        Self {
            proto: etcdserverpb::PutRequest {
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
    proto: etcdserverpb::PutResponse,
}

impl From<etcdserverpb::PutResponse> for PutResponse {
    fn from(resp: etcdserverpb::PutResponse) -> Self {
        Self { proto: resp }
    }
}

pub struct RangeRequest {
    proto: etcdserverpb::RangeRequest,
}

impl RangeRequest {
    fn new(key: Vec<u8>, range_end: Vec<u8>) -> Self {
        Self {
            proto: etcdserverpb::RangeRequest {
                key: key,
                range_end: range_end,
                limit: 0,
                revision: 0,
                sort_order: 0,
                sort_target: 0,
                serializable: false,
                keys_only: false,
                count_only: false,
                min_mod_revision: 0,
                max_mod_revision: 0,
                min_create_revision: 0,
                max_create_revision: 0,
            },
        }
    }

    pub fn all() -> Self {
        Self::new(vec![0], vec![0])
    }

    pub fn get<K>(key: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        Self::new(key.into(), vec![])
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

        Self::new(key, range_end)
    }

    pub fn range<K, V>(key: K, range_end: V) -> Self
    where
        K: Into<Vec<u8>>,
        V: Into<Vec<u8>>,
    {
        Self::new(key.into(), range_end.into())
    }

    pub fn set_limit(&mut self, limit: usize) {
        self.proto.limit = limit as i64;
    }
}

#[derive(Debug)]
pub struct RangeResponse {
    proto: etcdserverpb::RangeResponse,
}

impl RangeResponse {
    pub fn take_kvs(&mut self) -> Vec<KeyValue> {
        let kvs = std::mem::take(&mut self.proto.kvs);

        kvs.into_iter().map(From::from).collect()
    }

    pub fn has_more(&self) -> bool {
        self.proto.more
    }

    pub fn count(&self) -> usize {
        self.proto.count as usize
    }
}

impl From<etcdserverpb::RangeResponse> for RangeResponse {
    fn from(resp: etcdserverpb::RangeResponse) -> Self {
        Self { proto: resp }
    }
}
