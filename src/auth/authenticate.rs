use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug, Clone)]
pub struct AuthenticateRequest {
    proto: crate::proto::etcdserverpb::AuthenticateRequest,
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

impl From<etcdserverpb::AuthenticateRequest> for AuthenticateRequest {
    fn from(proto: etcdserverpb::AuthenticateRequest) -> Self {
        Self { proto }
    }
}

impl From<AuthenticateRequest> for etcdserverpb::AuthenticateRequest {
    fn from(value: AuthenticateRequest) -> Self {
        value.proto
    }
}

impl<N, P> From<(N, P)> for AuthenticateRequest
where
    N: Into<String>,
    P: Into<String>,
{
    fn from((user, password): (N, P)) -> Self {
        Self::new(user, password)
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticateResponse {
    pub header: ResponseHeader,
    pub token: String,
}

impl From<etcdserverpb::AuthenticateResponse> for AuthenticateResponse {
    fn from(proto: etcdserverpb::AuthenticateResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            token: proto.token,
        }
    }
}
