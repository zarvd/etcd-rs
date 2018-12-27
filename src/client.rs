use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

pub struct Client {}

impl Client {}

pub struct ClientBuilder {
    endpoints: Vec<String>,
}

impl ClientBuilder {
    pub fn endpoints(mut self, endpoints: Vec<String>) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn build(self) -> Client {
        let env = Arc::new(EnvBuilder::new().build());
        let addrs = self.endpoints.join(",");
        let ch = ChannelBuilder::new(env).connect(&addrs);
        let client = crate::proto::rpc_grpc::ClusterClient::new(ch);
        Client {}
    }
}
