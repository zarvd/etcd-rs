use std::sync::Arc;

use futures::Stream;

use crate::client::Inner;
use crate::watch::{Watch, WatchRequest, WatchResponse};
use crate::Error;

#[derive(Clone)]
pub struct WatchClient {
    inner: Arc<Inner>,
}

impl WatchClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub fn watch(&self, req: WatchRequest) -> impl Stream<Item = WatchResponse, Error = Error> {
        let (sink, receiver) = self.inner.watch.watch().unwrap();

        Watch::new(sink, receiver, req)
    }
}
