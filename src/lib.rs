mod client;
mod error;
mod proto;
mod response_header;

pub mod cluster;
pub mod kv;
pub mod lease;
pub mod lock;
pub mod watch;

pub use crate::{
    client::Client, client::ClientBuilder, cluster::ClusterClient, error::Error, kv::KvClient,
    lease::LeaseClient, lock::LockClient, response_header::ResponseHeader, watch::WatchClient,
};

pub mod prelude {
    pub use crate::kv::{
        DeleteRequest, DeleteResponse, GetRequest, GetResponse, KeyValue, PutRequest, PutResponse,
        TxnCmp, TxnOp, TxnRequest, TxnResponse, TxnResult,
    };

    pub use crate::lease::{
        GrantRequest, GrantResponse, KeepAliveRequest, KeepAliveResponse, RevokeRequest,
        RevokeResponse, TtlRequest, TtlResponse,
    };

    pub use crate::watch::{WatchRequest, WatchResponse};

    pub use crate::response_header::ResponseHeader;

    pub use crate::lock::{LockRequest, LockResponse, UnlockRequest, UnlockResponse};
}
