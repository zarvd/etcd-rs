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
//!         auth: None,
//!         tls: None,
//!     }).await?;
//!
//!     // print out all received watch responses
//!     let mut tunnel = client.watch().watch(KeyRange::key("foo")).await;
//!     tokio::spawn(async move {
//!         while let Some(resp) = tunnel.inbound().next().await {
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

use futures::future::FutureExt;
use futures::stream;
use futures::Stream;
use tokio::sync::{mpsc::channel, oneshot};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
};

pub use watch::{WatchCancelRequest, WatchCreateRequest, WatchResponse};

use crate::proto::etcdserverpb::watch_client::WatchClient;
use crate::proto::mvccpb;
use crate::Error;
use crate::KeyValue;

mod watch;

#[derive(Debug)]
pub enum WatchInbound {
    Ready(WatchResponse),
    Interrupted(Error),
    Closed,
}

/// WatchTunnel is a reusable connection for `Watch` operation
/// The underlying gRPC method is Bi-directional streaming
pub struct WatchTunnel {
    inbound: ReceiverStream<WatchInbound>,
    cancel: Option<oneshot::Sender<()>>,
}

impl WatchTunnel {
    fn new<F: 'static + Interceptor + Clone + Sync + Send>(
        mut client: WatchClient<InterceptedService<Channel, F>>,
        req: WatchCreateRequest,
    ) -> Self {
        let (resp_sender, resp_receiver) = channel(1024);

        let (cancel_tx, cancel_rx) = oneshot::channel();

        // monitor inbound watch response and transfer to the receiver
        tokio::spawn(async move {
            let mut inbound = {
                match client.watch(stream::iter(vec![req.into()])).await {
                    Ok(resp) => resp.into_inner(),
                    Err(e) => {
                        resp_sender
                            .send(WatchInbound::Interrupted(e.into()))
                            .await
                            .unwrap();
                        return;
                    }
                }
            };

            let mut cancel = cancel_rx.fuse();
            loop {
                let resp = futures::select! {
                    resp = inbound.message().fuse() => resp,
                    _ = cancel => { break; },
                };
                match resp {
                    Ok(Some(resp)) => {
                        resp_sender
                            .send(WatchInbound::Ready(resp.into()))
                            .await
                            .unwrap();
                    }
                    Ok(None) => {
                        break;
                    }
                    Err(e) => {
                        resp_sender
                            .send(WatchInbound::Interrupted(e.into()))
                            .await
                            .unwrap();
                    }
                };
            }
            resp_sender.send(WatchInbound::Closed).await.unwrap();
        });

        Self {
            inbound: ReceiverStream::new(resp_receiver),
            cancel: Some(cancel_tx),
        }
    }

    pub fn inbound(&mut self) -> &mut (dyn Stream<Item = WatchInbound> + Unpin + Send + Sync) {
        &mut self.inbound
    }
}

impl Drop for WatchTunnel {
    fn drop(&mut self) {
        let _ = self.cancel.take().unwrap().send(());
    }
}

/// Watch client.
#[derive(Clone)]
pub struct Watch<F: 'static + Interceptor + Clone + Sync + Send> {
    client: WatchClient<InterceptedService<Channel, F>>,
}

impl<F: 'static + Interceptor + Clone + Sync + Send> Watch<F> {
    pub(crate) fn new(client: WatchClient<InterceptedService<Channel, F>>) -> Self {
        Self { client }
    }

    /// Performs a watch operation.
    pub async fn watch<Req>(&mut self, req: Req) -> WatchTunnel
    where
        Req: Into<WatchCreateRequest>,
    {
        WatchTunnel::new(self.client.clone(), req.into())
    }
}

/// The kind of event.
#[derive(Debug, PartialEq)]
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
    pub fn take_kv(&mut self) -> Option<KeyValue> {
        self.proto.kv.take().map(From::from)
    }
}

impl From<mvccpb::Event> for Event {
    fn from(event: mvccpb::Event) -> Self {
        Self { proto: event }
    }
}
