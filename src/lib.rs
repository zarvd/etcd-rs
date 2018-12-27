mod client;
mod kv;
mod lease;
mod proto;

pub use crate::{client::Client, kv::Kv, lease::Lease};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
