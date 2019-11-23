//! An asynchronously etcd client for Rust.
//!
//! etcd-rs supports etcd v3 API and async/await syntax.
//!
//! # Examples
//!
//! A simple key-value read and write operation:
//!
//! ```no_run
//! use etcd_rs::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Client::new(ClientConfig {
//!         endpoints: vec!["http://127.0.0.1:2379".to_owned()],
//!         auth: None,
//!     });
//!
//!     let key = "foo";
//!     let value = "bar";
//!
//!     // Put a key-value pair
//!     let resp = client.kv().put(PutRequest::new(key, value)).await?;
//!
//!     println!("Put Response: {:?}", resp);
//!
//!     // Get the key-value pair
//!     let resp = client
//!         .kv()
//!         .range(RangeRequest::new(KeyRange::key(key)))
//!         .await?;
//!     println!("Range Response: {:?}", resp);
//!
//!     // Delete the key-valeu pair
//!     let resp = client
//!         .kv()
//!         .delete(DeleteRequest::new(KeyRange::key(key)))
//!         .await?;
//!     println!("Delete Response: {:?}", resp);
//!
//!    Ok(())
//! }
//! ```

mod auth;
mod client;
mod kv;
mod lease;
mod response_header;
mod watch;

pub(crate) mod proto {
    pub(crate) mod mvccpb {
        tonic::include_proto!("mvccpb");
    }

    pub(crate) mod authpb {
        tonic::include_proto!("authpb");
    }

    pub(crate) mod etcdserverpb {
        tonic::include_proto!("etcdserverpb");
    }

    pub(crate) mod v3lockpb {
        tonic::include_proto!("v3lockpb");
    }
}

pub use client::{Client, ClientConfig};
pub use kv::{
    DeleteRequest, DeleteResponse, KeyRange, KeyValue, Kv, PutRequest, PutResponse, RangeRequest,
    RangeResponse, TxnCmp, TxnRequest, TxnResponse,
};
pub use lease::{
    Lease, LeaseGrantRequest, LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse,
    LeaseRevokeRequest, LeaseRevokeResponse,
};
pub use auth::{Auth, AuthenticateRequest, AuthenticateResponse};
pub use response_header::ResponseHeader;
pub use watch::{Event, EventType, Watch, WatchRequest, WatchResponse};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
