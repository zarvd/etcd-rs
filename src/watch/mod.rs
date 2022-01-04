//! The Watch API provides an event-based interface for asynchronously monitoring changes to keys.
//!
//! # Examples
//!
//! Watch key `foo` changes
//!
//! ```no_run
//! use futures::StreamExt;
//!
//! use etcd_rs::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Client::connect(ClientConfig {
//!         endpoints: vec!["http://127.0.0.1:2379".to_owned()],
//!         ..Default::default()
//!     }).await?;
//!
//!     // print out all received watch responses
//!     let mut inbound = client.watch(KeyRange::key("foo")).await.unwrap();
//!     tokio::spawn(async move {
//!         while let Some(resp) = inbound.next().await {
//!             println!("watch response: {:?}", resp);
//!         }
//!     });
//!
//!     let key = "foo";
//!     client.kv().put(PutRequest::new(key, "bar")).await?;
//!     client.kv().put(PutRequest::new(key, "baz")).await?;
//!     client
//!         .kv()
//!         .delete(DeleteRequest::new(KeyRange::key(key)))
//!         .await?;
//!
//!     // not necessary, but will cleanly shut down the long-running tasks
//!     // spawned by the client
//!     client.shutdown().await;
//!
//!     Ok(())
//! }
//!
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use futures::future::FutureExt;
use futures::Stream;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
};

pub use watch::{WatchCancelRequest, WatchCreateRequest, WatchResponse};

use crate::lazy::{Lazy, Shutdown};
use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::watch_client::WatchClient;
use crate::proto::mvccpb;
use crate::Error;
use crate::KeyValue;
use crate::Result;
use std::sync::Mutex;

mod watch;

/// WatchTunnel is a reusable connection for `Watch` operation
/// The underlying gRPC method is Bi-directional streaming
struct WatchTunnel {
    req_sender: Option<UnboundedSender<etcdserverpb::WatchRequest>>,
    resp_receiver: Option<UnboundedReceiver<Result<Option<WatchResponse>>>>,
    shutdown: Option<oneshot::Sender<()>>,
}

impl WatchTunnel {
    fn new<F: 'static + Interceptor + Clone + Sync + Send>(
        mut client: WatchClient<InterceptedService<Channel, F>>,
    ) -> Self {
        let (req_sender, req_receiver) = unbounded_channel::<etcdserverpb::WatchRequest>();
        let (resp_sender, resp_receiver) = unbounded_channel::<Result<Option<WatchResponse>>>();

        let request = tonic::Request::new(UnboundedReceiverStream::new(req_receiver));
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        // monitor inbound watch response and transfer to the receiver
        tokio::spawn(async move {
            let mut shutdown_rx = shutdown_rx.fuse();
            let mut inbound = futures::select! {
                res = client.watch(request).fuse() => {
                    match res {
                        Err(e) => {
                            resp_sender.send(Err(From::from(e))).unwrap();
                            return;
                        },
                        Ok(i) => i.into_inner(),
                    }
                },
                _ = shutdown_rx => {
                    resp_sender.send(Ok(None)).unwrap();
                    return;
                },
            };

            loop {
                let resp = futures::select! {
                    resp = inbound.message().fuse() => resp,
                    _ = shutdown_rx => { break; },
                };
                match resp {
                    Ok(Some(resp)) => {
                        resp_sender.send(Ok(Some(resp.into()))).unwrap();
                    }
                    Ok(None) => {
                        break;
                    }
                    Err(e) => {
                        resp_sender.send(Err(e.into())).unwrap();
                        return;
                    }
                };
            }
            resp_sender.send(Ok(None)).unwrap();
        });

        Self {
            req_sender: Some(req_sender),
            resp_receiver: Some(resp_receiver),
            shutdown: Some(shutdown_tx),
        }
    }
}

#[async_trait]
impl Shutdown for WatchTunnel {
    async fn shutdown(&mut self) -> Result<()> {
        self.req_sender.take().ok_or(Error::ChannelClosed)?;
        self.shutdown.take().ok_or(Error::ChannelClosed)?;
        Ok(())
    }
}

/// Watch client.
#[derive(Clone)]
pub struct Watch<F: 'static + Interceptor + Clone + Sync + Send> {
    client: WatchClient<InterceptedService<Channel, F>>,
    tunnels: Arc<Mutex<Vec<Lazy<WatchTunnel>>>>,
}

impl<F: 'static + Interceptor + Clone + Sync + Send> Watch<F> {
    pub(crate) fn new(client: WatchClient<InterceptedService<Channel, F>>) -> Self {
        Self {
            client,
            tunnels: Arc::new(Mutex::new(vec![])),
        }
    }

    /// Performs a watch operation.
    pub async fn watch<C>(
        &mut self,
        req: C,
    ) -> Result<impl Stream<Item = Result<Option<WatchResponse>>>>
    where
        C: Into<WatchCreateRequest>,
    {
        let tunnel = {
            let client = self.client.clone();
            Lazy::new(move || WatchTunnel::new(client.clone()))
        };
        tunnel
            .write()
            .await
            .req_sender
            .as_mut()
            .ok_or(Error::ChannelClosed)?
            .send(req.into().into())
            .map_err(|_| Error::ChannelClosed)?;

        let recv = tunnel.write().await.resp_receiver.take().unwrap();
        let mut guard = self.tunnels.lock().unwrap();
        (*guard).push(tunnel);
        Ok(UnboundedReceiverStream::new(recv))
    }

    /// Shut down the running watch task, if any.
    pub async fn shutdown(&mut self) -> Result<()> {
        // If we implemented `Shutdown` for this, callers would need it in scope in
        // order to call this method.
        let mut guard = self.tunnels.lock().unwrap();
        while let Some(tunnel) = (*guard).pop() {
            let _ = tunnel.evict().await;
        }
        Ok(())
    }
}

/// The kind of event.
pub enum EventType {
    Put,
    Delete,
}

impl From<mvccpb::event::EventType> for EventType {
    fn from(event_type: mvccpb::event::EventType) -> Self {
        use mvccpb::event::EventType;
        match event_type {
            EventType::Put => Self::Put,
            EventType::Delete => Self::Delete,
        }
    }
}

/// Every change to every key is represented with Event messages.
pub struct Event {
    proto: mvccpb::Event,
}

impl Event {
    /// Gets the kind of event.
    pub fn event_type(&self) -> EventType {
        match self.proto.r#type {
            0 => EventType::Put,
            _ => EventType::Delete, // FIXME: assert valid event type
        }
    }

    /// Takes the key-value pair out of response, leaving a `None` in its place.
    pub fn take_kvs(&mut self) -> Option<KeyValue> {
        self.proto.kv.take().map(From::from)
    }
}

impl From<mvccpb::Event> for Event {
    fn from(event: mvccpb::Event) -> Self {
        Self { proto: event }
    }
}
