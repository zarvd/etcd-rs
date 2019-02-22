use std::sync::Arc;

use futures::Future;

use crate::client::Inner;
use crate::kv::{
    DeleteRequest, DeleteResponse, GetRequest, GetResponse, PutRequest, PutResponse, TxnRequest,
    TxnResponse,
};
use crate::Error;

#[derive(Clone)]
pub struct KvClient {
    inner: Arc<Inner>,
}

impl KvClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub fn put(&self, req: PutRequest) -> impl Future<Item = PutResponse, Error = Error> {
        self.inner
            .kv
            .put_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }

    pub fn delete(&self, req: DeleteRequest) -> impl Future<Item = DeleteResponse, Error = Error> {
        self.inner
            .kv
            .delete_range_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }

    pub fn get(&self, req: GetRequest) -> impl Future<Item = GetResponse, Error = Error> {
        self.inner
            .kv
            .range_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }

    pub fn txn(&self, req: TxnRequest) -> impl Future<Item = TxnResponse, Error = Error> {
        self.inner
            .kv
            .txn_async(&req.into())
            .unwrap()
            .map(From::from)
            .map_err(Error::GrpcFailure)
    }
}
