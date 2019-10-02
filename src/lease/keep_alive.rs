use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Clone)]
pub struct KeepAliveRequest {
    id: i64,
}

impl KeepAliveRequest {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl Into<rpc::LeaseKeepAliveRequest> for KeepAliveRequest {
    fn into(self) -> rpc::LeaseKeepAliveRequest {
        let mut req = rpc::LeaseKeepAliveRequest::new();
        req.set_ID(self.id);
        req
    }
}

#[derive(Debug)]
pub struct KeepAliveResponse {
    resp: rpc::LeaseKeepAliveResponse,
}

impl KeepAliveResponse {
    pub fn header(&self) -> ResponseHeader {
	    self.resp.get_header().into()
    }

    pub fn id(&self) -> i64 {
        self.resp.get_ID()
    }

    pub fn ttl(&self) -> i64 {
        self.resp.get_TTL()
    }
}

impl From<rpc::LeaseKeepAliveResponse> for KeepAliveResponse {
    fn from(resp: rpc::LeaseKeepAliveResponse) -> Self {
        Self { resp }
    }
}

// pub struct KeepAlive {
//     sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
//     receiver: grpcio::ClientDuplexReceiver<rpc::LeaseKeepAliveResponse>,
//     req: rpc::LeaseKeepAliveRequest,
// }

// impl KeepAlive {
//     pub fn new(
//         sender: grpcio::ClientDuplexSender<rpc::LeaseKeepAliveRequest>,
//         receiver: grpcio::ClientDuplexReceiver<rpc::LeaseKeepAliveResponse>,
//         req: KeepAliveRequest,
//     ) -> Self {
//         Self {
//             sender,
//             receiver,
//             req: req.into(),
//         }
//     }
// }

// impl Stream for KeepAlive {
//     type Item = KeepAliveResponse;
//     type Error = Error;

//     fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
//     }
// }
