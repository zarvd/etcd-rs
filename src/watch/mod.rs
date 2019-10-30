mod watch;
pub use watch::{WatchRequest, WatchResponse};

use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::WatchClient;
use crate::Result;

#[derive(Clone)]
pub struct Watch {
    client: WatchClient<Channel>,
}

impl Watch {
    pub(crate) fn new(client: WatchClient<Channel>) -> Self {
        Self { client }
    }

    // pub async fn watch(&mut self, req: Watch) -> Result<WatchResponse> {
    //     let resp = self.client.watch(tonic::Request::new(req.into())).await?;

    //     Ok(From::from(resp.into_inner().message()))
    // }
}
