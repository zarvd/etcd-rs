mod client;
mod cluster;
mod error;
mod kv;
mod lease;
mod proto;
mod response_header;

pub use crate::{
    client::Client, cluster::ClusterClient, error::Error, kv::KvClient, lease::Lease,
    response_header::ResponseHeader,
};

pub mod prelude {
    pub use crate::kv::{DeleteRequest, GetRequest, PutRequest};
}
