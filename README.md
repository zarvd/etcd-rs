etcd for Rust
====

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Travis Build Status][travis-badge]][travis-url]
[![Dependency Status][deps-badge]][deps-url]

[crates-badge]: https://img.shields.io/crates/v/etcd-rs.svg
[crates-url]: https://crates.io/crates/etcd-rs
[docs-badge]: https://docs.rs/etcd-rs/badge.svg
[docs-url]: https://docs.rs/etcd-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE-MIT
[travis-badge]: https://travis-ci.org/ccc13/etcd-rs.svg?branch=master
[travis-url]: https://travis-ci.org/ccc13/etcd-rs
[deps-badge]: https://deps.rs/repo/github/ccc13/etcd-rs/status.svg
[deps-url]: https://deps.rs/repo/github/ccc13/etcd-rs


An [etcd](https://github.com/etcd-io/etcd)(API v3) client for Rust, and it provides `async/await` APIs backed by [tokio](https://github.com/tokio-rs/tokio) and [tonic](https://github.com/hyperium/tonic).

Documentation on the library can be found at [docs.rs/etcd-rs](https://docs.rs/etcd-rs).

**NOTE**: Branch [master](https://github.com/ccc13/etcd-rs/tree/master) is currently under development. For v0.1.x based releases please check out the [v0.1.x](https://github.com/ccc13/etcd-rs/tree/v0.1.x) branch.

Features
----

- Asynchronous
- Etcd APIv3

Examples
----

[./examples](./examples)

Usage
----

Add following dependencies in your project `cargo.toml`:

```toml
[dependencies]
etcd-rs = "0.2.0-alpha.1"
```

#### Setup Client

```rust
let endpoints = vec!["http://127.0.0.1:2379".to_owned()];

let client = Client::new(ClientConfig {
    endpoints,
    auth: None,
});
```

#### Key Value

##### Put

```rust
client.kv().put(PutRequest::new("foo", "bar")).await?;
```

##### Get

Get a key-value pair

```rust
let resp = client.kv().get(KeyRange::key("foo")).await?;
```

List key-value paris by prefix

```rust
let resp = client.kv().get(KeyRange::prefix("foo")).await?;
```

List all key-value paris

```rust
let resp = client.kv().get(KeyRange::all()).await?;
```

##### Delete

Delete a key-value pair

```rust
client.kv().delete(KeyRange::key("foo")).await?;
```

Delete key-value paris by prefix

```rust
client.kv().delete(KeyRange::prefix("foo")).await?;
```

Delete all key-value paris

```rust
client.kv().delete(KeyRange::all()).await?;
```

#### Lease

##### Grant

```rust
let lease = client
    .lease()
    .grant(LeaseGrantRequest::new(Duration::from_secs(3)))
    .await?;
        
println!("lease id: {}", lease.id());

// Put key-value with the above lease
client
    .kv()
    .put({
        let mut req = PutRequest::new(key, "bar");
        req.set_lease(lease.id());

        req
    })
    .await?;
```

##### Revoke

```rust
client.lease().revoke(LeaseRevokeRequest::new(lease_id)).await?;
```

##### Keep Alive

```rust
use tokio::timer::Interval;
let mut interval = Interval::new_interval(Duration::from_secs(1));

loop {
    interval.next().await;
    client
        .lease()
        .keep_alive(LeaseKeepAliveRequest::new(lease_id))
        .await;
}
```

License
----

This project is licensed under the [MIT license](LICENSE).
