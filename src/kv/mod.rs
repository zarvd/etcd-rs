mod client;
mod delete;
mod get;
mod key_value;
mod put;

pub use crate::kv::{
    client::KvClient,
    delete::{DeleteRequest, DeleteResponse},
    get::{GetRequest, GetResponse},
    key_value::KeyValue,
    put::{PutRequest, PutResponse},
};

use crate::proto;

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    PUT,
    DELETE,
}

impl From<proto::kv::Event_EventType> for EventType {
    fn from(event_type: proto::kv::Event_EventType) -> Self {
        match event_type {
            proto::kv::Event_EventType::PUT => EventType::PUT,
            proto::kv::Event_EventType::DELETE => EventType::DELETE,
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
            field_type: From::from(event.get_field_type()),
            kv: From::from(event.take_kv()),
            prev_kv: From::from(event.take_prev_kv()),
        }
    }
}
