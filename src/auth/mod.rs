mod authenticate;

pub use authenticate::{AuthenticateRequest, AuthenticateResponse};

use async_trait::async_trait;

use crate::Result;

#[async_trait]
pub trait AuthOp {
    async fn authenticate<R>(&self, req: R) -> Result<AuthenticateResponse>
    where
        R: Into<AuthenticateRequest> + Send;
}
