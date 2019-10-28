use std::sync::Arc;
use tonic::transport::Channel;

use crate::kv::Kv;
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

pub(crate) struct Inner {
    pub channel: Channel,
    pub kv_client: Kv,
    // pub auth_client: AuthClient<Channel>,
    // pub cluster_client: ClusterClient<Channel>,
    // pub kv_client: KvClient<Channel>,
    // pub lease_client: LeaseClient<Channel>,
    // pub maintenance_client: MaintenanceClient<Channel>,
    // pub watch_client: WatchClient<Channel>,
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
            // let auth_client = AuthClient::new(channel.clone());
            let kv_client = Kv::new(KvClient::new(channel.clone()));
            // let cluster_client = ClusterClient::new(channel.clone());
            // let lease_client = LeaseClient::new(channel.clone());
            // let maintenance_client = MaintenanceClient::new(channel.clone());
            // let watch_client = WatchClient::new(channel.clone());
            Inner {
                channel,
                // auth_client,
                // cluster_client,
                kv_client,
                // lease_client,
                // maintenance_client,
                // watch_client,
            }
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn kv(&self) -> Kv {
        self.inner.kv_client.clone()
    }
}
