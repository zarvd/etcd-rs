use std::sync::Arc;

use futures::Future;

use crate::client::Inner;
use crate::proto::rpc::{DeleteRangeRequest, PutRequest, RangeRequest};

pub struct KvClient {
    inner: Arc<Inner>,
}

impl KvClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub fn put(&self, key: &str, value: &str) -> impl Future<Item = (), Error = ()> {
        let req = {
            let mut req = PutRequest::new();
            req.set_key(Vec::from(key));
            req.set_value(Vec::from(value));
            req
        };
        self.inner
            .kv
            .put_async(&req)
            .unwrap()
            .map(|_| ())
            .map_err(|_| ())
    }

    pub fn delete(&self, key: &str) -> impl Future<Item = (), Error = ()> {
        let req = {
            let mut req = DeleteRangeRequest::new();
            req.set_key(Vec::from(key));
            req
        };

        self.inner
            .kv
            .delete_range_async(&req)
            .unwrap()
            .map(|_| ())
            .map_err(|_| ())
    }

    pub fn get(&self, key: &str) -> impl Future<Item = Vec<(String, String)>, Error = ()> {
        let req = {
            let mut req = RangeRequest::new();
            req.set_key(Vec::from(key));
            req
        };
        self.inner
            .kv
            .range_async(&req)
            .unwrap()
            .map(|mut resp| {
                let kvs = resp.take_kvs().into_vec();
                let mut result = Vec::with_capacity(kvs.len());

                for kv in kvs {
                    let key = std::str::from_utf8(&kv.key).unwrap();
                    let value = std::str::from_utf8(&kv.value).unwrap();
                    result.push((key.to_owned(), value.to_owned()));
                }

                result
            })
            .map_err(|e| {
                println!("{}", e);
                ()
            })
    }
}
