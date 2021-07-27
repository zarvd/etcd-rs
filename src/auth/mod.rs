mod authenticate;

pub use authenticate::{AuthenticateRequest, AuthenticateResponse};

use tonic::transport::Channel;

use crate::client::Interceptor;
use crate::proto::etcdserverpb::auth_client::AuthClient;
use crate::Result;

/// Auth client.
#[derive(Clone)]
pub struct Auth {
    client: AuthClient<Channel>,
    interceptor: Interceptor,
}

impl Auth {
    pub(crate) fn new(client: AuthClient<Channel>, interceptor: Interceptor) -> Self {
        Self {
            client,
            interceptor,
        }
    }

    /// Performs an authenticating operation.
    /// It generates an authentication token based on a given user name and password.
    /// # Errors
    /// Will returns `Err` if the status of `response` is not `ok`
    pub async fn authenticate(&mut self, req: AuthenticateRequest) -> Result<AuthenticateResponse> {
        let resp = self
            .client
            .authenticate(self.interceptor.intercept(tonic::Request::new(req.into())))
            .await?;

        Ok(resp.into_inner().into())
    }
}
