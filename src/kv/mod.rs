mod compact;
mod delete;
mod put;
mod range;
mod txn;

pub use compact::{CompactRequest, CompactResponse};
pub use delete::{DeleteRequest, DeleteResponse};
pub use put::{PutRequest, PutResponse};
pub use range::{RangeRequest, RangeResponse};
pub use txn::{TxnCmp, TxnOp, TxnOpResponse, TxnRequest, TxnResponse};

use std::ops::Range;

use async_trait::async_trait;

use crate::lease::LeaseId;
use crate::proto::mvccpb;
use crate::Result;

#[async_trait]
pub trait KeyValueOp {
    async fn put<R>(&self, req: R) -> Result<PutResponse>
    where
        R: Into<PutRequest> + Send;

    async fn get<R>(&self, req: R) -> Result<RangeResponse>
    where
        R: Into<RangeRequest> + Send;
    async fn get_all(&self) -> Result<RangeResponse>;
    async fn get_by_prefix<K>(&self, p: K) -> Result<RangeResponse>
    where
        K: Into<Vec<u8>> + Send;
    async fn get_range<F, E>(&self, from: F, end: E) -> Result<RangeResponse>
    where
        F: Into<Vec<u8>> + Send,
        E: Into<Vec<u8>> + Send;

    async fn delete<R>(&self, req: R) -> Result<DeleteResponse>
    where
        R: Into<DeleteRequest> + Send;
    async fn delete_all(&self) -> Result<DeleteResponse>;
    async fn delete_by_prefix<K>(&self, p: K) -> Result<DeleteResponse>
    where
        K: Into<Vec<u8>> + Send;
    async fn delete_range<F, E>(&self, from: F, end: E) -> Result<DeleteResponse>
    where
        F: Into<Vec<u8>> + Send,
        E: Into<Vec<u8>> + Send;

    async fn txn<R>(&self, req: R) -> Result<TxnResponse>
    where
        R: Into<TxnRequest> + Send;

    async fn compact<R>(&self, req: R) -> Result<CompactResponse>
    where
        R: Into<CompactRequest> + Send;
}

/// Key-Value pair.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct KeyValue {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub create_revision: i64,
    pub mod_revision: i64,
    pub version: i64,
    pub lease: LeaseId,
}

impl KeyValue {
    /// Converts the key from bytes `&[u8]` to `&str`.
    /// Leaves the original `&[u8]` in place, and creates a new string slice containing the entire content.
    pub fn key_str(&self) -> &str {
        std::str::from_utf8(&self.key).expect("convert bytes to string")
    }

    /// Converts the value from bytes `&[u8]` to `&str`.
    /// Leaves the original `&[u8]` in place, and creates a new string slice containing the entire content.
    pub fn value_str(&self) -> &str {
        std::str::from_utf8(&self.value).expect("convert bytes to string")
    }
}

impl From<mvccpb::KeyValue> for KeyValue {
    fn from(proto: mvccpb::KeyValue) -> Self {
        Self {
            key: proto.key,
            value: proto.value,
            create_revision: proto.create_revision,
            mod_revision: proto.mod_revision,
            version: proto.version,
            lease: proto.lease,
        }
    }
}

/// KeyRange is an abstraction for describing etcd key of various types.
#[derive(Clone, Hash, PartialEq, Eq)]
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
