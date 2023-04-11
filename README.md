etcd client for Rust
====

[<img alt="github" height="20" src="https://img.shields.io/badge/github-lodrem/etcd--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/lodrem/etcd-rs)
[<img alt="crates.io" height="20" src="https://img.shields.io/crates/v/etcd-rs.svg?style=for-the-badge&color=fc8d62&logo=rust">](https://crates.io/crates/etcd-rs)
[<img alt="docs.rs" height="20" src="https://img.shields.io/badge/docs.rs-etcd--rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white">](https://docs.rs/etcd-rs)
[<img alt="build status" height="20" src="https://img.shields.io/github/actions/workflow/status/lodrem/etcd-rs/ci.yml?branch=master&style=for-the-badge">](https://github.com/luncj/etcd-rs/actions?query%3Amaster)
[<img alt="dependency status" height="20" src="https://deps.rs/repo/github/lodrem/etcd-rs/status.svg?style=for-the-badge">](https://deps.rs/repo/github/lodrem/etcd-rs)

An [etcd](https://github.com/etcd-io/etcd) (API v3) client for Rust backed by [tokio](https://github.com/tokio-rs/tokio) and [tonic](https://github.com/hyperium/tonic).

Supported APIs
----

- KV
  - [x] Put
  - [x] Range
  - [x] Delete
  - [x] Transaction
  - [x] Compact
- Lease
  - [x] Grant
  - [x] Revoke
  - [x] KeepAlive
  - [x] TimeToLive
- Watch
  - [x] WatchCreate
  - [x] WatchCancel
- Auth
  - [x] Authenticate
  - [ ] RoleAdd
  - [ ] RoleGrantPermission
  - [ ] UserAdd
  - [ ] UserGrantRole
  - [ ] AuthEnable
  - [ ] AuthDisable
- Cluster
  - [x] MemberAdd
  - [x] MemberRemove
  - [x] MemberUpdate
  - [x] MemberList
- Maintenance
  - [ ] Alarm
  - [ ] Status
  - [ ] Defragment
  - [ ] Hash
  - [ ] Snapshot
  - [ ] MoveLeader

Usage
----

Add following dependencies in your project `cargo.toml`:

```toml
[dependencies]
etcd-rs = "1.0"
```

```rust
use etcd_rs::Client;

#[tokio::main]
async fn main() {
    let cli = Client::connect(ClientConfig {
        endpoints: [
            "http://127.0.0.1:12379",
            "http://127.0.0.1:22379",
            "http://127.0.0.1:32379",
        ],
        ..Default::default()
    }).await;
    
    cli.put(("foo", "bar")).await.expect("put kv");
    
    let kvs = cli.get("foo").await.expect("get kv").take_kvs();
    assert_eq!(kvs.len(), 1);
}
```

Development
----


requirements:
- Makefile
- docker
- docker-compose

### Start local etcd cluster

```shell
make setup-etcd-cluster
```

stop cluster
```shell
make teardown-etcd-cluster
```

### Run tests

```shell
make test
```

for specified case:
```shell
TEST_CASE=test_put_error make test-one
```

License
----

This project is licensed under the [MIT license](LICENSE).
