#![feature(mem_take)]
mod client;
mod kv;

pub mod proto {
    pub mod mvccpb {
        tonic::include_proto!("mvccpb");
    }

    pub mod authpb {
        tonic::include_proto!("authpb");
    }

    pub mod etcdserverpb {
        tonic::include_proto!("etcdserverpb");
    }

    pub mod v3lockpb {
        tonic::include_proto!("v3lockpb");
    }
}

pub use client::{Client, ClientConfig};
pub use kv::{
    DeleteRequest, DeleteResponse, KeyRange, KeyValue, Kv, PutRequest, PutResponse, RangeRequest,
    RangeResponse, TxnCmp, TxnRequest, TxnResponse,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
