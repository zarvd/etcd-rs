Examples
====

## for Non-TLS

```shell
cd ./etcd-rs
make setup-etcd-cluster

cargo run --example kv
cargo run --example watch
```

## for TLS

```shell
cd ./etcd-rs
ETCD_CLUSTER_WITH_TLS=true make setup-etcd-cluster

cargo run --example tls
```

