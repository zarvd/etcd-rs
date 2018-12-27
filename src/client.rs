use std::sync::Arc;

use crate::proto::rpc_grpc::{
    AuthClient, ClusterClient, KvClient, LeaseClient, MaintenanceClient, WatchClient,
};
use grpcio::{Channel, ChannelBuilder, EnvBuilder};

pub struct Client {
    inner: Arc<Inner>,
}

impl Client {
    pub fn cluster(&self) -> &ClusterClient {
        &self.inner.cluster
    }

    pub fn kv(&self) -> &KvClient {
        &self.inner.kv
    }

    pub fn auth(&self) -> &AuthClient {
        &self.inner.auth
    }

    pub fn lease(&self) -> &LeaseClient {
        &self.inner.lease
    }

    pub fn watch(&self) -> &WatchClient {
        &self.inner.watch
    }

    pub fn maintenance(&self) -> &MaintenanceClient {
        &self.inner.maintenance
    }
}

pub struct ClientBuilder {
    endpoints: Vec<String>,
    auth: Option<(String, String)>,
}

impl ClientBuilder {
    pub fn endpoints(mut self, endpoints: Vec<String>) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn auth<N>(mut self, username: N, password: N) -> Self
    where
        N: Into<String>,
    {
        self.auth = Some((username.into(), password.into()));
        self
    }

    pub fn build(self) -> Client {
        let env = Arc::new(EnvBuilder::new().build());
        let addrs = self.endpoints.join(",");
        let channel = ChannelBuilder::new(env).connect(&addrs);

        let (username, password) = {
            match self.auth {
                Some((username, password)) => (Some(username), Some(password)),
                _ => (None, None),
            }
        };

        let cluster = ClusterClient::new(channel.clone());
        let kv = KvClient::new(channel.clone());
        let auth = AuthClient::new(channel.clone());
        let lease = LeaseClient::new(channel.clone());
        let watch = WatchClient::new(channel.clone());
        let maintenance = MaintenanceClient::new(channel.clone());

        let inner = Arc::new(Inner {
            cluster,
            kv,
            auth,
            lease,
            watch,
            maintenance,
            username,
            password,
            channel,
        });

        Client { inner }
    }
}

/// TODO Balancer
pub(crate) struct Inner {
    cluster: ClusterClient,
    kv: KvClient,
    auth: AuthClient,
    lease: LeaseClient,
    watch: WatchClient,
    maintenance: MaintenanceClient,

    username: Option<String>,
    password: Option<String>,

    channel: Channel,
}

impl Inner {}
