mod authenticate;

pub use authenticate::{AuthenticateRequest, AuthenticateResponse};

use tonic::transport::Channel;

use crate::proto::etcdserverpb::auth_client::AuthClient;
use crate::Result;

/// Auth client.
#[derive(Clone)]
pub struct Auth {
    client: AuthClient<Channel>,
}

impl Auth {
    pub(crate) fn new(client: AuthClient<Channel>) -> Self {
        Self { client }
    }

    /// Performs an authenticating operation.
    /// It generates an authentication token based on a given user name and password.
    pub async fn authenticate(&mut self, req: AuthenticateRequest) -> Result<AuthenticateResponse> {
        let resp = self
            .client
            .authenticate(tonic::Request::new(req.into()))
            .await?;

        Ok(From::from(resp.into_inner()))
    }
}
