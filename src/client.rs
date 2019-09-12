use std::sync::Arc;

use grpcio::{Channel, ChannelBuilder, EnvBuilder};

use crate::proto::lock_grpc::LockClient as LockClientProto;
use crate::proto::rpc_grpc::{
    AuthClient as AuthClientProto, ClusterClient as ClusterClientProto, KvClient as KvClientProto,
    LeaseClient as LeaseClientProto, MaintenanceClient as MaintenanceClientProto,
    WatchClient as WatchClientProto,
};

use crate::{ClusterClient, KvClient, LeaseClient, LockClient, WatchClient};

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

impl Client {
    pub fn cluster(&self) -> ClusterClient {
        ClusterClient::new(self.inner.clone())
    }

    pub fn kv(&self) -> KvClient {
        KvClient::new(self.inner.clone())
    }

    pub fn auth(&self) -> &AuthClientProto {
        &self.inner.auth
    }

    pub fn lease(&self) -> LeaseClient {
        LeaseClient::new(self.inner.clone())
    }

    pub fn watch(&self) -> WatchClient {
        WatchClient::new(self.inner.clone())
    }

    pub fn maintenance(&self) -> &MaintenanceClientProto {
        &self.inner.maintenance
    }

    pub fn lock(&self) -> LockClient {
        LockClient::new(self.inner.clone())
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            endpoints: Default::default(),
            auth: None,
            max_receive_msg_len: None,
            max_send_msg_len: None,
        }
    }
}

pub struct ClientBuilder {
    endpoints: Vec<String>,
    auth: Option<(String, String)>,
    max_receive_msg_len: Option<i32>,
    max_send_msg_len: Option<i32>,
}

impl ClientBuilder {
    pub fn endpoints(mut self, endpoints: Vec<String>) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn add_endpoint<N>(mut self, endpoint: N) -> Self
    where
        N: Into<String>,
    {
        self.endpoints.push(endpoint.into());
        self
    }

    pub fn auth<N>(mut self, username: N, password: N) -> Self
    where
        N: Into<String>,
    {
        self.auth = Some((username.into(), password.into()));
        self
    }

    pub fn max_receive_msg_len(mut self, len: i32) -> Self {
        self.max_receive_msg_len = Some(len);
        self
    }

    pub fn max_send_msg_len(mut self, len: i32) -> Self {
        self.max_send_msg_len = Some(len);
        self
    }

    pub fn build(self) -> Client {
        let env = Arc::new(EnvBuilder::new().build());
        let addrs = self.endpoints.join(",");

        let mut channel = ChannelBuilder::new(env);

        if let Some(len) = self.max_receive_msg_len {
            channel.max_receive_message_len(len);
        }

        if let Some(len) = self.max_send_msg_len {
            channel.max_send_message_len(len);
        }

        let channel = channel.connect(&addrs);

        let (username, password) = match self.auth {
            Some((username, password)) => (Some(username), Some(password)),
            _ => (None, None),
        };

        let cluster = ClusterClientProto::new(channel.clone());
        let kv = KvClientProto::new(channel.clone());
        let auth = AuthClientProto::new(channel.clone());
        let lease = LeaseClientProto::new(channel.clone());
        let watch = WatchClientProto::new(channel.clone());
        let maintenance = MaintenanceClientProto::new(channel.clone());
        let lock = LockClientProto::new(channel.clone());

        let inner = Arc::new(Inner {
            cluster,
            kv,
            auth,
            lease,
            watch,
            maintenance,
            lock,
            username,
            password,
            channel,
        });

        Client { inner }
    }
}

/// TODO Balancer
pub(crate) struct Inner {
    pub cluster: ClusterClientProto,
    pub kv: KvClientProto,
    pub auth: AuthClientProto,
    pub lease: LeaseClientProto,
    pub watch: WatchClientProto,
    pub maintenance: MaintenanceClientProto,
    pub lock: LockClientProto,

    pub username: Option<String>,
    pub password: Option<String>,

    pub channel: Channel,
}

impl Inner {}
