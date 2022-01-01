#![allow(
clippy::suspicious_op_assign_impl,
clippy::suspicious_arithmetic_impl,
clippy::module_inception
)]
#![deny(
clippy::clone_on_ref_ptr,
clippy::dbg_macro,
clippy::enum_glob_use,
clippy::get_unwrap,
clippy::macro_use_imports
)]

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
//!     let client = Client::connect(ClientConfig {
//!         endpoints: vec!["http://127.0.0.1:2379".to_owned()],
//!         ..Default::default()
//!     }).await?;
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

pub use auth::{Auth, AuthenticateRequest, AuthenticateResponse};
pub use client::{Client, ClientConfig};
pub use error::Error;
pub use kv::{
    DeleteRequest, DeleteResponse, KeyRange, KeyValue, Kv, PutRequest, PutResponse, RangeRequest,
    RangeResponse, TxnCmp, TxnOp, TxnOpResponse, TxnRequest, TxnResponse,
};
pub use lease::{
    Lease, LeaseGrantRequest, LeaseGrantResponse, LeaseKeepAliveRequest, LeaseKeepAliveResponse,
    LeaseRevokeRequest, LeaseRevokeResponse,
};
pub use response_header::ResponseHeader;
pub use watch::{Event, EventType, Watch, WatchCancelRequest, WatchCreateRequest, WatchResponse};

macro_rules! pbwrap_request {
    ($(#[$attr:meta])* $intern:ident => $name:ident) => {
        $(#[$attr])*
        pub struct $name {
            proto: crate::proto::etcdserverpb::$intern,
        }
        impl From<$name> for crate::proto::etcdserverpb::$intern {
            fn from(x: $name) -> Self {
                x.proto
            }
        }
    };
    ($(#[$attr:meta])* $name:ident) => {
        pbwrap_request!($(#[$attr])* $name => $name);
    }
}

macro_rules! pbwrap_response {
    ($(#[$attr:meta])* $intern:ident => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug)]
        pub struct $name {
            proto: crate::proto::etcdserverpb::$intern,
        }
        impl From<crate::proto::etcdserverpb::$intern> for $name {
            fn from(resp: crate::proto::etcdserverpb::$intern) -> Self {
                Self { proto: resp }
            }
        }
    };
    ($(#[$attr:meta])* $name:ident) => {
        pbwrap_response!($(#[$attr])* $name => $name);
    }
}

mod auth;
mod client;
mod error;
mod kv;
mod lazy;
mod lease;
mod proto;
mod response_header;
mod watch;

pub type Result<T> = std::result::Result<T, Error>;
