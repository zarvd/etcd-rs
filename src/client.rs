use std::sync::Arc;

use tokio::stream::Stream;
use tonic::{metadata::MetadataValue, transport::Channel, Interceptor, Request};

use crate::proto::etcdserverpb::{
    auth_client::AuthClient, kv_client::KvClient, lease_client::LeaseClient,
    watch_client::WatchClient,
};
use crate::watch::WatchResponse;
use crate::{Auth, KeyRange, Kv, Lease, Result as Res, Watch};
use tonic::transport::ClientTlsConfig;

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
    auth_client: Auth,
    kv_client: Kv,
    watch_client: Watch,
    lease_client: Lease,
}

impl Client {
    /// Connects to etcd generate auth token.
    /// The client connection used to request the authentication token is typically thrown away; it cannot carry the new token’s credentials. This is because gRPC doesn’t provide a way for adding per RPC credential after creation of the connection
    async fn generate_auth_token(cfg: &ClientConfig) -> Res<Option<String>> {
        use crate::AuthenticateRequest;

        let endpoints = &cfg.endpoints;
        let channel = {
            let endpoints = endpoints
                .into_iter()
                .map(|e| Channel::from_shared(e.to_owned()).expect("parse endpoint URI"));
            Channel::balance_list(endpoints)
        };

        let mut auth_client = Auth::new(AuthClient::new(channel));

        match cfg.auth.as_ref() {
            Some((name, password)) => {
                auth_client
                    .authenticate(AuthenticateRequest::new(name, password))
                    .await
                    .and_then(|r| Ok(Some(r.token().to_owned())))
            }
            None => Ok(None)
        }
    }

    /// Connects to etcd cluster and returns a client.
    pub async fn connect(cfg: ClientConfig) -> Res<Self> {
        // If authentication provided, geneartes token before connecting.
        let token = Self::generate_auth_token(&cfg).await?;

        let auth_interceptor = if let Some(token) = token {
            let token = MetadataValue::from_str(&token).unwrap();
            Some(Interceptor::new(move |mut req: Request<()>| {
                req.metadata_mut().insert("authorization", token.clone());

                Ok(req)
            }))
        } else {
            None
        };

        let channel = {
            let endpoints = cfg
                .endpoints
                .into_iter()
                .map(|e| Channel::from_shared(e).expect("parse endpoint URI"));
            Channel::balance_list(endpoints)
        };

        let inner = {
            let (auth_client, kv_client, watch_client, lease_client) =
                if let Some(auth_interceptor) = auth_interceptor {
                    (
                        Auth::new(AuthClient::with_interceptor(
                            channel.clone(),
                            auth_interceptor.clone(),
                        )),
                        Kv::new(KvClient::with_interceptor(
                            channel.clone(),
                            auth_interceptor.clone(),
                        )),
                        Watch::new(WatchClient::with_interceptor(
                            channel.clone(),
                            auth_interceptor.clone(),
                        )),
                        Lease::new(LeaseClient::with_interceptor(
                            channel.clone(),
                            auth_interceptor,
                        )),
                    )
                } else {
                    (
                        Auth::new(AuthClient::new(channel.clone())),
                        Kv::new(KvClient::new(channel.clone())),
                        Watch::new(WatchClient::new(channel.clone())),
                        Lease::new(LeaseClient::new(channel.clone())),
                    )
                };
            Inner {
                channel,
                auth_client,
                kv_client,
                watch_client,
                lease_client,
            }
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
    ) -> impl Stream<Item = Result<WatchResponse, tonic::Status>> {
        let mut client = self.inner.watch_client.clone();
        client.watch(key_range).await
    }

    /// Gets a lease client.
    pub fn lease(&self) -> Lease {
        self.inner.lease_client.clone()
    }

    /// Shut down any running tasks.
    pub async fn shutdown(&self) -> Res<()> {
        let mut watch_client = self.inner.watch_client.clone();
        watch_client.shutdown().await?;
        let mut lease_client = self.inner.lease_client.clone();
        lease_client.shutdown().await?;
        Ok(())
    }
}
