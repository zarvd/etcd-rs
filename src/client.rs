use std::sync::Arc;

use futures::Stream;
use tonic::metadata::Ascii;
use tonic::transport::ClientTlsConfig;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

use crate::proto::etcdserverpb::{
    auth_client::AuthClient, kv_client::KvClient, lease_client::LeaseClient,
    watch_client::WatchClient,
};
use crate::watch::WatchResponse;
use crate::{Auth, KeyRange, Kv, Lease, Result, Watch};

/// Config for establishing etcd client.
#[derive(Clone)]
pub struct ClientConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<(String, String)>,
    pub tls: Option<ClientTlsConfig>,
}

/// Client is an abstraction for grouping etcd operations and managing underlying network communications.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

#[derive(Clone)]
pub(crate) struct Interceptor {
    token: Option<MetadataValue<Ascii>>,
}

impl Interceptor {
    pub(crate) fn intercept<T>(&self, mut req: Request<T>) -> Request<T> {
        if let Some(token) = self.token.as_ref() {
            req.metadata_mut().insert("authorization", token.clone());
        }
        req
    }
}

#[allow(dead_code)]
pub(crate) struct Inner {
    channel: Channel,
    auth_client: Auth,
    kv_client: Kv,
    watch_client: Watch,
    lease_client: Lease,
}

impl Client {
    fn get_channel(cfg: &ClientConfig) -> Result<Channel> {
        let mut endpoints = Vec::with_capacity(cfg.endpoints.len());
        for e in cfg.endpoints.iter() {
            let c = Channel::from_shared(e.to_owned())?;
            endpoints.push(match &cfg.tls {
                Some(tls) => c.tls_config(tls.to_owned())?,
                None => c,
            });
        }
        Ok(Channel::balance_list(endpoints.into_iter()))
    }

    /// Connects to etcd generate auth token.
    /// The client connection used to request the authentication token is typically thrown away; it cannot carry the new token’s credentials. This is because gRPC doesn’t provide a way for adding per RPC credential after creation of the connection
    async fn generate_auth_token(cfg: &ClientConfig) -> Result<Option<String>> {
        use crate::AuthenticateRequest;

        let channel = Self::get_channel(&cfg)?;

        let mut auth_client = Auth::new(AuthClient::new(channel), Interceptor { token: None });

        let token = match cfg.auth.as_ref() {
            Some((name, password)) => auth_client
                .authenticate(AuthenticateRequest::new(name, password))
                .await
                .map(|r| Some(r.token().to_owned()))?,
            None => None,
        };

        Ok(token)
    }

    /// Connects to etcd cluster and returns a client.
    ///
    /// # Errors
    /// Will returns `Err` if failed to contact with given endpoints or authentication failed.
    pub async fn connect(cfg: ClientConfig) -> Result<Self> {
        // If authentication provided, generates token before connecting.
        let token = Self::generate_auth_token(&cfg).await?;
        let token = token.map(|token| MetadataValue::from_str(&token).unwrap());

        let interceptor = Interceptor { token };

        let channel = Self::get_channel(&cfg)?;

        let inner = Inner {
            auth_client: Auth::new(AuthClient::new(channel.clone()), interceptor.clone()),
            kv_client: Kv::new(KvClient::new(channel.clone()), interceptor.clone()),
            watch_client: Watch::new(WatchClient::new(channel.clone()), interceptor.clone()),
            lease_client: Lease::new(LeaseClient::new(channel.clone()), interceptor.clone()),
            channel,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Gets an auth client.
    pub fn auth(&self) -> Auth {
        self.inner.auth_client.clone()
    }

    /// Gets a key-value client.
    pub fn kv(&self) -> Kv {
        self.inner.kv_client.clone()
    }

    /// Gets a watch client.
    pub fn watch_client(&self) -> Watch {
        self.inner.watch_client.clone()
    }

    /// Perform a watch operation
    pub async fn watch(
        &self,
        key_range: KeyRange,
    ) -> Result<impl Stream<Item = Result<Option<WatchResponse>>>> {
        let mut wc = self.watch_client();
        wc.watch(key_range).await?;
        Ok(wc.take_receiver().await)
    }

    /// Gets a lease client.
    pub fn lease(&self) -> Lease {
        self.inner.lease_client.clone()
    }

    /// Shut down any running tasks.
    pub async fn shutdown(&self) -> Result<()> {
        let mut watch_client = self.inner.watch_client.clone();
        watch_client.shutdown().await?;
        let mut lease_client = self.inner.lease_client.clone();
        lease_client.shutdown().await?;
        Ok(())
    }
}
