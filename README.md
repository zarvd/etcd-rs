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
}).await;
```

if authenticate enabled

```rust
let endpoints = vec!["http://127.0.0.1:2379".to_owned()];

let client = Client::connect(ClientConfig {
    endpoints,
    auth: Some(("user".to_owned(), "password".to_owned())),
}).await;
```

License
----

This project is licensed under the [MIT license](LICENSE).
