use tonic::transport::Channel;

use crate::proto::etcdserverpb as pb;
use crate::proto::etcdserverpb::client::KvClient;
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

pub struct RangeRequest {
    proto: pb::RangeRequest,
}

impl RangeRequest {
    fn new(key: Vec<u8>, range_end: Vec<u8>) -> Self {
        Self {
            proto: pb::RangeRequest {
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
    proto: pb::RangeResponse,
}

impl From<pb::RangeResponse> for RangeResponse {
    fn from(resp: pb::RangeResponse) -> Self {
        Self { proto: resp }
    }
}
