mod client;
mod cluster;
mod kv;
mod lease;
mod proto;

pub use crate::{client::Client, cluster::Cluster, kv::KvClient, lease::Lease};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
