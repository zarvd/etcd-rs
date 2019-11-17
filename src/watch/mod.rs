//! The Watch API provides an event-based interface for asynchronously monitoring changes to keys.
//!
//! # Examples
//!
//! Watch key `foo` changes
//!
//! ```no_run
//! use tokio::prelude::*;
//!
//! use etcd_rs::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Client::new(ClientConfig {
//!         endpoints: vec!["http://127.0.0.1:2379".to_owned()],
//!         auth: None,
//!     });
//!
//!     // print out all received watch responses
//!     let mut inbound = client.watch().responses();
//!     tokio::spawn(async move {
//!         loop {
//!             let resp = inbound.next().await.unwrap();
//!             println!("watch response: {:?}", resp);
//!         }
//!     });
//!
//!     client
//!         .watch()
//!         .watch(WatchRequest::create(KeyRange::key("foo")))
//!         .await;
//!
//!     let key = "foo";
//!     client.kv().put(PutRequest::new(key, "bar")).await?;
//!     client.kv().put(PutRequest::new(key, "baz")).await?;
//!     client
//!         .kv()
//!         .delete(DeleteRequest::new(KeyRange::key(key)))
//!         .await?;
//!
//!     Ok(())
//! }
//!
//! ```

mod watch;
pub use watch::{WatchRequest, WatchResponse};

use std::sync::{Arc, RwLock};

use tokio::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tonic::transport::Channel;

use crate::proto::etcdserverpb;
use crate::proto::etcdserverpb::client::WatchClient;
use crate::proto::mvccpb;
use crate::KeyValue;

/// WatchTunnel is a reusable connection for `Watch` operation
/// The underlying gRPC method is Bi-directional streaming
struct WatchTunnel {
    req_sender: UnboundedSender<WatchRequest>,
    resp_receiver: Option<UnboundedReceiver<Result<WatchResponse, tonic::Status>>>,
}

impl WatchTunnel {
    fn new(mut client: WatchClient<Channel>) -> Self {
        let (req_sender, mut req_receiver) = unbounded_channel::<WatchRequest>();
        let (mut resp_sender, resp_receiver) =
            unbounded_channel::<Result<WatchResponse, tonic::Status>>();

        let request = tonic::Request::new(async_stream::stream! {
            while let Some(req) = req_receiver.next().await {
                let pb: etcdserverpb::WatchRequest = req.into();
                yield pb;
            }
        });

        // monitor inbound watch response and transfer to the receiver
        tokio::spawn(async move {
            let mut inbound = client.watch(request).await.unwrap().into_inner();

            loop {
                let resp = inbound.message().await;
                match resp {
                    Ok(Some(resp)) => {
                        resp_sender.send(Ok(From::from(resp))).await.unwrap();
                    }
                    Ok(None) => {
                        return;
                    }
                    Err(e) => {
                        resp_sender.send(Err(e)).await.unwrap();
                    }
                };
            }
        });

        Self {
            req_sender,
            resp_receiver: Some(resp_receiver),
        }
    }

    fn take_resp_receiver(&mut self) -> UnboundedReceiver<Result<WatchResponse, tonic::Status>> {
        self.resp_receiver
            .take()
            .expect("take the unique watch response receiver")
    }
}

/// Watch client.
#[derive(Clone)]
pub struct Watch {
    client: WatchClient<Channel>,
    tunnel: Arc<RwLock<WatchTunnel>>,
}

impl Watch {
    pub(crate) fn new(client: WatchClient<Channel>) -> Self {
        let tunnel = Arc::new(RwLock::new(WatchTunnel::new(client.clone())));

        Self { client, tunnel }
    }

    /// Fetch response stream.
    pub fn responses(&mut self) -> impl Stream<Item = Result<WatchResponse, tonic::Status>> {
        self.tunnel.write().unwrap().take_resp_receiver()
    }

    /// Performs a watch operation.
    pub async fn watch(&mut self, req: WatchRequest) {
        self.tunnel
            .write()
            .unwrap()
            .req_sender
            .send(req)
            .await
            .unwrap();
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
        match self.proto.kv.take() {
            Some(kv) => Some(From::from(kv)),
            _ => None,
        }
    }
}

impl From<mvccpb::Event> for Event {
    fn from(event: mvccpb::Event) -> Self {
        Self { proto: event }
    }
}
