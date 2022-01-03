mod delete;
mod put;
mod range;
mod txn;

pub use delete::{DeleteRequest, DeleteResponse};
pub use put::{PutRequest, PutResponse};
pub use range::{RangeRequest, RangeResponse};
use std::ops::Range;
pub use txn::{TxnCmp, TxnOp, TxnOpResponse, TxnRequest, TxnResponse};

use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
};

use crate::proto::etcdserverpb::kv_client::KvClient;
use crate::proto::mvccpb;
use crate::Result as Res;

/// Key-Value client.
#[derive(Clone)]
pub struct Kv<F> {
    client: KvClient<InterceptedService<Channel, F>>,
}

impl<F: Interceptor + Clone> Kv<F> {
    pub(crate) fn new(client: KvClient<InterceptedService<Channel, F>>) -> Self {
        Self { client }
    }

    /// Performs a key-value saving operation.
    pub async fn put(&mut self, req: PutRequest) -> Res<PutResponse> {
        let resp = self.client.put(tonic::Request::new(req.into())).await?;

        Ok(resp.into_inner().into())
    }

    /// Performs a key-value fetching operation.
    pub async fn range(&mut self, req: RangeRequest) -> Res<RangeResponse> {
        let resp = self.client.range(tonic::Request::new(req.into())).await?;

        Ok(resp.into_inner().into())
    }

    /// Performs a key-value deleting operation.
    pub async fn delete(&mut self, req: DeleteRequest) -> Res<DeleteResponse> {
        let resp = self
            .client
            .delete_range(tonic::Request::new(req.into()))
            .await?;

        Ok(resp.into_inner().into())
    }

    /// Performs a transaction operation.
    pub async fn txn(&mut self, req: TxnRequest) -> Res<TxnResponse> {
        let resp = self.client.txn(tonic::Request::new(req.into())).await?;

        Ok(resp.into_inner().into())
    }
}

/// Key-Value pair.
#[derive(Clone, PartialEq)]
pub struct KeyValue {
    proto: mvccpb::KeyValue,
}

impl KeyValue {
    /// Gets the key in bytes. An empty key is not allowed.
    pub fn key(&self) -> &[u8] {
        &self.proto.key
    }

    /// Takes the key out of response, leaving an empty vector in its place.
    pub fn take_key(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.proto.key)
    }

    /// Converts the key from bytes `&[u8]` to `&str`.
    /// Leaves the original `&[u8]` in place, and creates a new string slice containing the entire content.
    pub fn key_str(&self) -> &str {
        std::str::from_utf8(&self.proto.key).expect("convert bytes to string")
    }

    /// Gets the value held by the key, in bytes.
    pub fn value(&self) -> &[u8] {
        &self.proto.value
    }

    /// Takes the value out of response, leaving an empty vector in its place.
    pub fn take_value(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.proto.value)
    }

    /// Converts the value from bytes `&[u8]` to `&str`.
    /// Leaves the original `&[u8]` in place, and creates a new string slice containing the entire content.
    pub fn value_str(&self) -> &str {
        std::str::from_utf8(&self.proto.value).expect("convert bytes to string")
    }

    /// Gets the revision of last creation on this key.
    pub fn create_revision(&self) -> usize {
        self.proto.create_revision as usize
    }

    /// Gets the revision of last modification on this key.
    pub fn mod_revision(&self) -> usize {
        self.proto.mod_revision as usize
    }

    /// Gets the version of the key.
    pub fn version(&self) -> usize {
        self.proto.version as usize
    }

    /// Gets the ID of the lease that attached to key.
    pub fn lease(&self) -> usize {
        self.proto.lease as usize
    }

    /// Returns `true` if this KeyValue has a lease attached, and `false` otherwise.
    pub fn has_lease(&self) -> bool {
        self.proto.lease != 0
    }
}

impl From<mvccpb::KeyValue> for KeyValue {
    fn from(kv: mvccpb::KeyValue) -> Self {
        Self { proto: kv }
    }
}

/// KeyRange is an abstraction for describing etcd key of various types.
pub struct KeyRange {
    pub key: Vec<u8>,
    pub range_end: Vec<u8>,
}

impl KeyRange {
    /// Creates a new KeyRange for describing a range of multiple keys.
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

    /// Creates a new KeyRange for describing a specified key.
    pub fn key<K>(key: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        Self {
            key: key.into(),
            range_end: vec![],
        }
    }

    /// Creates a new KeyRange for describing all keys.
    pub fn all() -> Self {
        Self {
            key: vec![0],
            range_end: vec![0],
        }
    }

    /// Creates a new KeyRange for describing keys prefixed with specified value.
    pub fn prefix<K>(prefix: K) -> Self
    where
        K: Into<Vec<u8>>,
    {
        let key = prefix.into();
        if key.is_empty() {
            // An empty Vec<u8> results in an invalid KeyRange.
            // Assume that an empty value passed to this method implies no prefix (i.e., all keys).
            return KeyRange::all();
        }

        let range_end = {
            let mut end = key.clone();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end.truncate(i + 1);
                    break;
                }
            }
            end
        };
        Self { key, range_end }
    }
}

impl<T> From<Range<T>> for KeyRange
where
    T: Into<Vec<u8>>,
{
    fn from(range: Range<T>) -> Self {
        Self::range(range.start, range.end)
    }
}

impl From<&str> for KeyRange {
    fn from(k: &str) -> Self {
        Self::key(k)
    }
}

impl From<String> for KeyRange {
    fn from(k: String) -> Self {
        Self::key(k)
    }
}
