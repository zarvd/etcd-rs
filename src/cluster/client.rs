use std::sync::Arc;

use futures::Future;

use crate::client::Inner;
use crate::proto::rpc::MemberListRequest;

pub struct ClusterClient {
    inner: Arc<Inner>,
}

impl ClusterClient {
    pub(crate) fn new(inner: Arc<Inner>) -> Self {
        Self { inner }
    }

    pub fn member_list(&self) -> impl Future<Item = Vec<u64>, Error = ()> {
        let req = MemberListRequest::new();

        self.inner
            .cluster
            .member_list_async(&req)
            .unwrap()
            .map(|mut resp| {
                resp.take_members()
                    .into_vec()
                    .iter()
                    .map(|member| member.get_ID())
                    .collect()
            })
            .map_err(|e| {
                println!("{}", e);
                ()
            })
    }
}
