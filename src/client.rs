use async_trait::async_trait;
use std::time::Duration;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    codegen::InterceptedService,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, ClientTlsConfig},
    Request, Status,
};

use crate::auth::{AuthOp, AuthenticateRequest, AuthenticateResponse};
use crate::cluster::{
    ClusterOp, MemberAddRequest, MemberAddResponse, MemberListRequest, MemberListResponse,
    MemberRemoveRequest, MemberRemoveResponse, MemberUpdateRequest, MemberUpdateResponse,
};
use crate::kv::{
    CompactRequest, CompactResponse, DeleteRequest, DeleteResponse, KeyRange, KeyValueOp,
    PutRequest, PutResponse, RangeRequest, RangeResponse, TxnRequest, TxnResponse,
};
use crate::lease::{
    LeaseGrantRequest, LeaseGrantResponse, LeaseId, LeaseKeepAlive, LeaseOp, LeaseRevokeRequest,
    LeaseRevokeResponse, LeaseTimeToLiveRequest, LeaseTimeToLiveResponse,
};
use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::cluster_client::ClusterClient;
use crate::proto::etcdserverpb::maintenance_client::MaintenanceClient;
use crate::proto::etcdserverpb::{
    auth_client::AuthClient, kv_client::KvClient, lease_client::LeaseClient,
    watch_client::WatchClient,
};
use crate::watch::{WatchCanceler, WatchCreateRequest, WatchOp, WatchStream};
use crate::{Error, Result};

#[derive(Clone)]
pub struct TokenInterceptor {
    token: Option<MetadataValue<Ascii>>,
}

impl TokenInterceptor {
    fn new(token: Option<String>) -> Self {
        Self {
            token: token.map(|token: String| MetadataValue::from_str(&token).unwrap()),
        }
    }
}

impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> std::result::Result<Request<()>, Status> {
        match &self.token {
            Some(token) => {
                req.metadata_mut().insert("authorization", token.clone());
                Ok(req)
            }
            None => Ok(req),
        }
    }
}

/// Config for establishing etcd client.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<(String, String)>,
    pub tls: Option<ClientTlsConfig>,
    pub connect_timeout: Duration,
    pub http2_keep_alive_interval: Duration,
}

impl ClientConfig {
    pub fn new<I>(endpoints: I) -> Self
    where
        I: Into<Vec<String>>,
    {
        Self {
            endpoints: endpoints.into(),
            auth: None,
            tls: None,
            connect_timeout: Duration::from_secs(30),
            http2_keep_alive_interval: Duration::from_secs(5),
        }
    }

    pub fn with_auth<N, P>(mut self, name: N, password: P) -> Self
    where
        N: Into<String>,
        P: Into<String>,
    {
        self.auth = Some((name.into(), password.into()));
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn with_http2_keep_alive_interval(mut self, interval: Duration) -> Self {
        self.http2_keep_alive_interval = interval;
        self
    }
}

/// Client is an abstraction for grouping etcd operations and managing underlying network communications.
#[derive(Clone)]
pub struct Client {
    auth_client: AuthClient<InterceptedService<Channel, TokenInterceptor>>,
    kv_client: KvClient<InterceptedService<Channel, TokenInterceptor>>,
    watch_client: WatchClient<InterceptedService<Channel, TokenInterceptor>>,
    cluster_client: ClusterClient<InterceptedService<Channel, TokenInterceptor>>,
    maintenance_client: MaintenanceClient<InterceptedService<Channel, TokenInterceptor>>,
    lease_client: LeaseClient<InterceptedService<Channel, TokenInterceptor>>,
}

impl Client {
    pub async fn connect_with_token(cfg: &ClientConfig, token: Option<String>) -> Result<Self> {
        let auth_interceptor = TokenInterceptor::new(token);

        let channel = {
            let mut endpoints = Vec::with_capacity(cfg.endpoints.len());
            for e in cfg.endpoints.iter() {
                let c = Channel::from_shared(e.to_owned())?
                    .connect_timeout(cfg.connect_timeout)
                    .http2_keep_alive_interval(cfg.http2_keep_alive_interval);

                endpoints.push(match &cfg.tls {
                    Some(tls) => c.tls_config(tls.to_owned())?,
                    None => c,
                });
            }

            Channel::balance_list(endpoints.into_iter())
        };

        let auth_client = AuthClient::with_interceptor(channel.clone(), auth_interceptor.clone());
        let kv_client = KvClient::with_interceptor(channel.clone(), auth_interceptor.clone());
        let watch_client = WatchClient::with_interceptor(channel.clone(), auth_interceptor.clone());
        let cluster_client =
            ClusterClient::with_interceptor(channel.clone(), auth_interceptor.clone());
        let maintenance_client =
            MaintenanceClient::with_interceptor(channel.clone(), auth_interceptor.clone());
        let lease_client = LeaseClient::with_interceptor(channel, auth_interceptor);

        Ok(Self {
            auth_client,
            kv_client,
            watch_client,
            cluster_client,
            maintenance_client,
            lease_client,
        })
    }

    /// Connects to etcd cluster and returns a client.
    ///
    /// # Errors
    /// Will returns `Err` if failed to contact with given endpoints or authentication failed.
    pub async fn connect(mut cfg: ClientConfig) -> Result<Self> {
        let cli = Self::connect_with_token(&cfg, None).await?;

        match cfg.auth.take() {
            Some((name, password)) => {
                let token = cli.authenticate((name, password)).await?.token;

                Self::connect_with_token(&cfg, Some(token)).await
            }
            None => Ok(cli),
        }
    }
}

#[async_trait]
impl AuthOp for Client {
    async fn authenticate<R>(&self, req: R) -> Result<AuthenticateResponse>
    where
        R: Into<AuthenticateRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.auth_client.clone().authenticate(req).await?;

        Ok(resp.into_inner().into())
    }
}

#[async_trait]
impl KeyValueOp for Client {
    async fn put<R>(&self, req: R) -> Result<PutResponse>
    where
        R: Into<PutRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.kv_client.clone().put(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn get<R>(&self, req: R) -> Result<RangeResponse>
    where
        R: Into<RangeRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.kv_client.clone().range(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn get_all(&self) -> Result<RangeResponse> {
        self.get(KeyRange::all()).await
    }

    async fn get_by_prefix<K>(&self, p: K) -> Result<RangeResponse>
    where
        K: Into<Vec<u8>> + Send,
    {
        self.get(KeyRange::prefix(p)).await
    }

    async fn get_range<F, E>(&self, from: F, end: E) -> Result<RangeResponse>
    where
        F: Into<Vec<u8>> + Send,
        E: Into<Vec<u8>> + Send,
    {
        self.get(KeyRange::range(from, end)).await
    }

    async fn delete<R>(&self, req: R) -> Result<DeleteResponse>
    where
        R: Into<DeleteRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.kv_client.clone().delete_range(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn delete_all(&self) -> Result<DeleteResponse> {
        self.delete(KeyRange::all()).await
    }

    async fn delete_by_prefix<K>(&self, p: K) -> Result<DeleteResponse>
    where
        K: Into<Vec<u8>> + Send,
    {
        self.delete(KeyRange::prefix(p)).await
    }

    async fn delete_range<F, E>(&self, from: F, end: E) -> Result<DeleteResponse>
    where
        F: Into<Vec<u8>> + Send,
        E: Into<Vec<u8>> + Send,
    {
        self.delete(KeyRange::range(from, end)).await
    }

    async fn txn<R>(&self, req: R) -> Result<TxnResponse>
    where
        R: Into<TxnRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.kv_client.clone().txn(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn compact<R>(&self, req: R) -> Result<CompactResponse>
    where
        R: Into<CompactRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.kv_client.clone().compact(req).await?;

        Ok(resp.into_inner().into())
    }
}

#[async_trait]
impl WatchOp for Client {
    async fn watch<R>(&self, req: R) -> Result<(WatchStream, WatchCanceler)>
    where
        R: Into<WatchCreateRequest> + Send,
    {
        let (tx, rx) = channel::<etcdserverpb::WatchRequest>(128);

        tx.send(req.into().into()).await?;

        let resp = self
            .watch_client
            .clone()
            .watch(ReceiverStream::new(rx))
            .await?;

        let mut inbound = resp.into_inner();

        let watch_id = match inbound.message().await? {
            Some(resp) => {
                if !resp.created {
                    return Err(Error::WatchEvent(
                        "should receive created event at first".to_owned(),
                    ));
                }
                resp.watch_id
            }

            None => return Err(Error::CreateWatch),
        };

        Ok((WatchStream::new(inbound), WatchCanceler::new(watch_id, tx)))
    }

    // async fn cancel_watch<R>(&self, req: R) -> Result<()>
    // where
    //     R: Into<WatchCancelRequest> + Send,
    // {
    //     println!("canceling watch");
    //     let req = req.into().into();
    //     self.watch_client
    //         .clone()
    //         .watch(stream::iter(vec![req]))
    //         .await?;
    //
    //     println!("canceled watch");
    //     Ok(())
    // }
}

#[async_trait]
impl LeaseOp for Client {
    async fn grant_lease<R>(&self, req: R) -> Result<LeaseGrantResponse>
    where
        R: Into<LeaseGrantRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.lease_client.clone().lease_grant(req).await?;
        Ok(resp.into_inner().into())
    }

    async fn revoke<R>(&self, req: R) -> Result<LeaseRevokeResponse>
    where
        R: Into<LeaseRevokeRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.lease_client.clone().lease_revoke(req).await?;
        Ok(resp.into_inner().into())
    }

    async fn keep_alive_for(&self, lease_id: LeaseId) -> Result<LeaseKeepAlive> {
        let (req_tx, req_rx) = channel(1024);

        let req_rx = ReceiverStream::new(req_rx);

        let resp_rx = self
            .lease_client
            .clone()
            .lease_keep_alive(req_rx)
            .await?
            .into_inner();

        Ok(LeaseKeepAlive::new(lease_id, req_tx, resp_rx))
    }

    async fn time_to_live<R>(&self, req: R) -> Result<LeaseTimeToLiveResponse>
    where
        R: Into<LeaseTimeToLiveRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.lease_client.clone().lease_time_to_live(req).await?;
        Ok(resp.into_inner().into())
    }
}

#[async_trait]
impl ClusterOp for Client {
    async fn member_add<R>(&self, req: R) -> Result<MemberAddResponse>
    where
        R: Into<MemberAddRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.cluster_client.clone().member_add(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn member_remove<R>(&self, req: R) -> Result<MemberRemoveResponse>
    where
        R: Into<MemberRemoveRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.cluster_client.clone().member_remove(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn member_update<R>(&self, req: R) -> Result<MemberUpdateResponse>
    where
        R: Into<MemberUpdateRequest> + Send,
    {
        let req = tonic::Request::new(req.into().into());
        let resp = self.cluster_client.clone().member_update(req).await?;

        Ok(resp.into_inner().into())
    }

    async fn member_list(&self) -> Result<MemberListResponse> {
        let req = tonic::Request::new(MemberListRequest::new().into());
        let resp = self.cluster_client.clone().member_list(req).await?;

        Ok(resp.into_inner().into())
    }
}
