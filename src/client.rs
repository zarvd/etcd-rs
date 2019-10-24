use std::sync::Arc;
use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::{
    AuthClient, ClusterClient, KvClient, LeaseClient, MaintenanceClient, WatchClient,
};

pub struct ClientConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<(String, String)>,
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    channel: Channel,
    auth_client: AuthClient<Channel>,
    cluster_client: ClusterClient<Channel>,
    kv_client: KvClient<Channel>,
    lease_client: LeaseClient<Channel>,
    maintenance_client: MaintenanceClient<Channel>,
    watch_client: WatchClient<Channel>,
}

impl Client {
    pub fn new(cfg: ClientConfig) -> Self {
        let channel = {
            let endpoints = cfg
                .endpoints
                .into_iter()
                .map(|e| Channel::from_shared(&e[..]).expect("construct HTTP/2 channel"));
            Channel::balance_list(endpoints)
        };

        let inner = {
            let auth_client = AuthClient::new(channel.clone());
            let kv_client = KvClient::new(channel.clone());
            let cluster_client = ClusterClient::new(channel.clone());
            let lease_client = LeaseClient::new(channel.clone());
            let maintenance_client = MaintenanceClient::new(channel.clone());
            let watch_client = WatchClient::new(channel.clone());
            Inner {
                channel,
                auth_client,
                cluster_client,
                kv_client,
                lease_client,
                maintenance_client,
                watch_client,
            }
        };

        Self {
            inner: Arc::new(inner),
        }
    }
}
