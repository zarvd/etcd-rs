etcd for Rust
====

[![CI Status][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]

[ci-badge]: https://img.shields.io/github/workflow/status/luncj/etcd-rs/CI?style=flat-square
[ci-url]: https://github.com/luncj/etcd-rs/actions
[crates-badge]: https://img.shields.io/crates/v/etcd-rs.svg?style=flat-square
[crates-url]: https://crates.io/crates/etcd-rs
[license-badge]: https://img.shields.io/github/license/luncj/etcd-rs.svg?style=flat-square
[license-url]: https://github.com/luncj/etcd-rs/blob/master/LICENSE


An [etcd](https://github.com/etcd-io/etcd)(API v3) client for Rust, and it provides `async/await` APIs backed by [tokio](https://github.com/tokio-rs/tokio) and [tonic](https://github.com/hyperium/tonic).

Documentation on the library can be found at [docs.rs/etcd-rs](https://docs.rs/etcd-rs).

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
etcd-rs = "0.2"
```

#### Setup Client

```rust
let endpoints = vec!["http://127.0.0.1:2379".to_owned()];

let client = Client::connect(ClientConfig {
    endpoints,
    auth: None,
    tls: None
}).await;
```

if authenticate enabled

```rust
let endpoints = vec!["http://127.0.0.1:2379".to_owned()];

let client = Client::connect(ClientConfig {
    endpoints,
    auth: Some(("user".to_owned(), "password".to_owned())),
    tls: None
}).await;
```

with tls

```rust
let endpoints = vec!["https://127.0.0.1:2379".to_owned()];
let tls = ClientTlsConfig::new();

let client = Client::connect(ClientConfig {
    endpoints,
    auth: Some(("user".to_owned(), "password".to_owned())),
    tls: Some(tls)
}).await;
```

License
----

This project is licensed under the [MIT license](LICENSE).
