mod client;
mod delete;
mod get;
mod key_value;
mod put;
mod txn;

pub use crate::kv::{
    client::KvClient,
    delete::{DeleteRequest, DeleteResponse},
    get::{GetRequest, GetResponse},
    key_value::KeyValue,
    put::{PutRequest, PutResponse},
    txn::{TxnCmp, TxnOp, TxnRequest, TxnResponse, TxnResult},
};

use crate::proto;

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    Put,
    Delete,
}

impl From<proto::kv::Event_EventType> for EventType {
    fn from(event_type: proto::kv::Event_EventType) -> Self {
        match event_type {
            proto::kv::Event_EventType::PUT => EventType::Put,
            proto::kv::Event_EventType::DELETE => EventType::Delete,
        }
    }
}

#[derive(Debug)]
pub struct Event {
    field_type: EventType,
    kv: KeyValue,
    prev_kv: KeyValue,
}

impl Event {
    pub fn field_type(&self) -> EventType {
        self.field_type
    }

    pub fn kv(&self) -> &KeyValue {
        &self.kv
    }

    pub fn prev_kv(&self) -> &KeyValue {
        &self.prev_kv
    }
}

impl From<proto::kv::Event> for Event {
    fn from(mut event: proto::kv::Event) -> Event {
        Event {
	        field_type: event.field_type.into(),
            kv: event.take_kv().into(),
            prev_kv: event.take_prev_kv().into(),
        }
    }
}
