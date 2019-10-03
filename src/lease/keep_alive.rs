use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Clone)]
pub struct KeepAliveRequest {
    id: i64,
}

impl KeepAliveRequest {
    pub fn new(id: i64) -> Self {
        KeepAliveRequest { id }
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
    header: ResponseHeader,
    id: i64,
    ttl: i64,
}

impl KeepAliveResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ttl(&self) -> i64 {
        self.ttl
    }
}

impl From<rpc::LeaseKeepAliveResponse> for KeepAliveResponse {
    fn from(mut resp: rpc::LeaseKeepAliveResponse) -> Self {
        KeepAliveResponse {
            header: resp.take_header().into(),
            id: resp.ID,
            ttl: resp.TTL,
        }
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
