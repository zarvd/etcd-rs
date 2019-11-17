use std::sync::Arc;
use tonic::transport::Channel;

use crate::proto::etcdserverpb::client::{KvClient, LeaseClient, WatchClient};
use crate::{Kv, Lease, Watch};

/// Config for establishing etcd client.
pub struct ClientConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<(String, String)>,
}

/// Client is an abstraction for grouping etcd operations and managing underlying network communications.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

#[allow(dead_code)]
pub(crate) struct Inner {
    channel: Channel,
    kv_client: Kv,
    watch_client: Watch,
    lease_client: Lease,
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
            let kv_client = Kv::new(KvClient::new(channel.clone()));
            let watch_client = Watch::new(WatchClient::new(channel.clone()));
            let lease_client = Lease::new(LeaseClient::new(channel.clone()));
            Inner {
                channel,
                kv_client,
                watch_client,
                lease_client,
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
