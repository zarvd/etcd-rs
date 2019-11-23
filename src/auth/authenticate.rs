use crate::proto::etcdserverpb;
use crate::ResponseHeader;

/// Request for authenticating.
pub struct AuthenticateRequest {
    proto: etcdserverpb::AuthenticateRequest,
}

impl AuthenticateRequest {
    pub fn new<N, P>(name: N, password: P) -> Self
    where
        N: Into<String>,
        P: Into<String>,
    {
        let proto = etcdserverpb::AuthenticateRequest {
            name: name.into(),
            password: password.into(),
        };
        Self { proto }
    }
}

impl Into<etcdserverpb::AuthenticateRequest> for AuthenticateRequest {
    fn into(self) -> etcdserverpb::AuthenticateRequest {
        self.proto
    }
}

/// Response for authenticating.
#[derive(Debug)]
pub struct AuthenticateResponse {
    proto: etcdserverpb::AuthenticateResponse,
}

impl AuthenticateResponse {
    /// Takes the header out of response, leaving a `None` in its place.
    pub fn take_header(&mut self) -> Option<ResponseHeader> {
        match self.proto.header.take() {
            Some(header) => Some(From::from(header)),
            _ => None,
        }
    }

    /// Gets an authorized token that can be used in succeeding RPCs.
    pub fn token(&self) -> &str {
        &self.proto.token
    }
}

impl From<etcdserverpb::AuthenticateResponse> for AuthenticateResponse {
    fn from(resp: etcdserverpb::AuthenticateResponse) -> Self {
        Self { proto: resp }
    }
}
