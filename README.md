etcd client for Rust
====

[<img alt="github" height="20" src="https://img.shields.io/badge/github-lodrem/etcd--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/lodrem/etcd-rs)
[<img alt="crates.io" height="20" src="https://img.shields.io/crates/v/etcd-rs.svg?style=for-the-badge&color=fc8d62&logo=rust">](https://crates.io/crates/etcd-rs)
[<img alt="docs.rs" height="20" src="https://img.shields.io/badge/docs.rs-etcd--rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K">](https://docs.rs/etcd-rs)
[<img alt="build status" height="20" src="https://img.shields.io/github/workflow/status/lodrem/etcd-rs/CI/master?style=for-the-badge">](https://github.com/luncj/etcd-rs/actions?query%3Amaster)
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
