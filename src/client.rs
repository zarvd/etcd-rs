use std::sync::Arc;
use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::{
    AuthClient, ClusterClient, KvClient, LeaseClient, MaintenanceClient, WatchClient,
};
use crate::{Kv, Lease, Watch};

/// Config for establishing etcd client.
pub struct ClientConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<(String, String)>,
}

/// Client is a abstraction for grouping etcd operations and managing underlying network communications.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

pub struct Inner {
    channel: Channel,
    kv_client: Kv,
    watch_client: Watch,
    lease_client: Lease,
    // pub auth_client: AuthClient<Channel>,
    // pub cluster_client: ClusterClient<Channel>,
    // pub kv_client: KvClient<Channel>,
    // pub maintenance_client: MaintenanceClient<Channel>,
    // pub watch_client: WatchClient<Channel>,
}

impl Client {
    /// Creates a new client for etcd.
    pub fn new(cfg: ClientConfig) -> Self {
        let channel = {
            let endpoints = cfg
                .endpoints
                .into_iter()
                .map(|e| Channel::from_shared(&e[..]).expect("construct HTTP/2 channel"));
            Channel::balance_list(endpoints)
        };

        let inner = {
            // let auth_client = AuthClient::new(channel.clone());
            let kv_client = Kv::new(KvClient::new(channel.clone()));
            let watch_client = Watch::new(WatchClient::new(channel.clone()));
            // let cluster_client = ClusterClient::new(channel.clone());
            let lease_client = Lease::new(LeaseClient::new(channel.clone()));
            // let maintenance_client = MaintenanceClient::new(channel.clone());
            Inner {
                channel,
                // auth_client,
                // cluster_client,
                kv_client,
                watch_client,
                lease_client,
                // maintenance_client,
            }
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    /// Get key-value client.
    pub fn kv(&self) -> Kv {
        self.inner.kv_client.clone()
    }

    /// Get watch client.
    pub fn watch(&self) -> Watch {
        self.inner.watch_client.clone()
    }

    /// Get lease client.
    pub fn lease(&self) -> Lease {
        self.inner.lease_client.clone()
    }
}
