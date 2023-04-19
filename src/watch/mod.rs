//! The Watch API provides an event-based interface for asynchronously monitoring changes to keys.

mod watch;

pub use watch::{WatchCancelRequest, WatchCreateRequest, WatchResponse};

use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures::Stream;
use tokio::sync::mpsc::Sender;
use tonic::Streaming;

use crate::proto::etcdserverpb;
use crate::proto::mvccpb;
use crate::{Error, KeyValue, Result};

#[async_trait]
pub trait WatchOp {
    async fn watch<R>(&self, req: R) -> Result<WatchStream>
    where
        R: Into<WatchCreateRequest> + Send;
}

#[derive(Debug)]
pub enum WatchInbound {
    Ready(WatchResponse),
    Interrupted(Error),
    Closed,
}

pub struct WatchStream {
    watch_id: i64,
    req_tx: Sender<etcdserverpb::WatchRequest>,
    resp_rx: Streaming<etcdserverpb::WatchResponse>,
}

impl WatchStream {
    pub(crate) fn new(
        watch_id: i64,
        req_tx: Sender<etcdserverpb::WatchRequest>,
        resp_rx: Streaming<etcdserverpb::WatchResponse>,
    ) -> Self {
        Self {
            watch_id,
            req_tx,
            resp_rx,
        }
    }

    pub async fn inbound(&mut self) -> WatchInbound {
        match self.resp_rx.message().await {
            Ok(Some(resp)) => {
                if resp.canceled && resp.events.is_empty() {
                    WatchInbound::Closed
                } else {
                    WatchInbound::Ready(resp.into())
                }
            }
            Ok(None) => WatchInbound::Interrupted(Error::WatchEventExhausted),
            Err(e) => WatchInbound::Interrupted(e.into()),
        }
    }
}

impl Stream for WatchStream {
    type Item = WatchInbound;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().resp_rx)
            .poll_next(cx)
            .map(|e| match e {
                Some(Ok(resp)) => Some(WatchInbound::Ready(resp.into())),
                Some(Err(e)) => Some(WatchInbound::Interrupted(e.into())),
                None => Some(WatchInbound::Closed),
            })
    }
}

impl Drop for WatchStream {
    fn drop(&mut self) {
        let _ = self
            .req_tx
            .blocking_send(WatchCancelRequest::new(self.watch_id).into());
    }
}

/// The kind of event.
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub kv: KeyValue,
    pub prev_kv: Option<KeyValue>,
}

impl From<mvccpb::Event> for Event {
    fn from(proto: mvccpb::Event) -> Self {
        Self {
            event_type: match proto.r#type {
                0 => EventType::Put,
                _ => EventType::Delete, // FIXME: assert valid event type
            },
            kv: From::from(proto.kv.expect("must fetch kv")),
            prev_kv: proto.prev_kv.map(KeyValue::from),
        }
    }
}
