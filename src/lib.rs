mod client;
mod cluster;
mod error;
mod kv;
mod lease;
mod proto;
mod response_header;

pub use crate::{
    client::Client, cluster::ClusterClient, error::Error, kv::KvClient, lease::LeaseClient,
    response_header::ResponseHeader,
};

pub mod prelude {
    pub use crate::kv::{
        DeleteRequest, DeleteResponse, GetRequest, GetResponse, KvClient, PutRequest, PutResponse,
    };

    pub use crate::lease::{
        GrantRequest, GrantResponse, KeepAliveRequest, KeepAliveResponse, LeaseClient,
        RevokeRequest, RevokeResponse, TtlRequest, TtlResponse,
    };

    pub use crate::response_header::ResponseHeader;
}
