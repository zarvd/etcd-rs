#![allow(dead_code)]
#![allow(unused_macros)]

use etcd_rs::Endpoint;
use std::collections::HashMap;
use std::process::Command;

pub struct EtcdCluster {
    nodes: HashMap<String, Endpoint>,
}

impl EtcdCluster {
    pub fn new(with_tls: bool) -> Self {
        println!("etcd cluster starting");
        {
            let output = Command::new("make")
                .env("ETCD_CLUSTER_WITH_TLS", with_tls.to_string())
                .arg("setup-etcd-cluster")
                .output()
                .expect("setup etcd cluster");
            assert!(
                output.status.success(),
                "stdout: {} \nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        println!("etcd cluster started");

        let nodes: HashMap<_, _> = (1..=3)
            .map(|i| {
                let node = format!("etcd-{}", i);
                if with_tls {
                    use std::fs::read;

                    let ca_cert = read("./hack/certs/ca.pem").expect("read ca cert");
                    let client_cert =
                        read(format!("./hack/certs/{}.pem", node)).expect("read client cert");
                    let client_key =
                        read(format!("./hack/certs/{}-key.pem", node)).expect("read client key");
                    (
                        node.clone(),
                        Endpoint::from(format!("https://127.0.0.1:{}2379", i)).tls_raw(
                            node,
                            ca_cert,
                            client_cert,
                            client_key,
                        ),
                    )
                } else {
                    (node, format!("http://127.0.0.1:{}2379", i).into())
                }
            })
            .collect();

        Self { nodes }
    }

    pub fn print_status(&self) {
        let output = Command::new("make")
            .arg("etcd-cluster-status")
            .output()
            .expect("fetch etcd cluster status");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    pub fn endpoints(&self) -> Vec<Endpoint> {
        self.nodes.values().cloned().collect()
    }

    #[track_caller]
    pub fn start_node(&self, i: u64) {
        let caller = std::panic::Location::caller();
        println!(
            "[{}:{}] => etcd node {} starting",
            caller.file(),
            caller.line(),
            i
        );
        assert!(Command::new("make")
            .env("ETCD_NODE", format!("etcd-{}", i))
            .arg("start-etcd-node")
            .output()
            .expect("start etcd node")
            .status
            .success());
        println!(
            "[{}:{}] => etcd node {} started",
            caller.file(),
            caller.line(),
            i
        );
    }

    #[track_caller]
    pub fn stop_node(&self, i: u64) {
        let caller = std::panic::Location::caller();
        println!(
            "[{}:{}] => etcd node {} stopping",
            caller.file(),
            caller.line(),
            i
        );
        assert!(Command::new("make")
            .env("ETCD_NODE", format!("etcd-{}", i))
            .arg("stop-etcd-node")
            .output()
            .expect("stop etcd node")
            .status
            .success());
        println!(
            "[{}:{}] => etcd node {} stopped",
            caller.file(),
            caller.line(),
            i
        );
    }
}

impl Drop for EtcdCluster {
    fn drop(&mut self) {
        println!("etcd cluster stopping");
        assert!(Command::new("make")
            .arg("teardown-etcd-cluster")
            .output()
            .expect("teardown etcd cluster")
            .status
            .success());
        println!("etcd cluster stopped");
    }
}

pub struct Context {
    pub etcd_cluster: EtcdCluster,
    auth: Option<(String, String)>,
}

impl Context {
    pub fn new(with_tls: bool) -> Self {
        Self {
            etcd_cluster: EtcdCluster::new(with_tls),
            auth: None,
        }
    }

    pub fn set_auth(mut self, user: String, pwd: String) -> Self {
        self.auth = Some((user, pwd));
        self
    }

    pub async fn connect_to_cluster(&self) -> etcd_rs::Client {
        use etcd_rs::*;

        Client::connect(ClientConfig::new(self.etcd_cluster.endpoints()))
            .await
            .expect("connect to etcd cluster")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KVOp {
    Put(String, String),
    Delete(String),
}

macro_rules! apply_kv_ops {
    ($cli:expr, $ops:expr) => {
        for op in $ops.iter() {
            match op {
                KVOp::Put(k, v) => {
                    let resp = $cli.put(PutRequest::new(k.clone(), v.clone())).await;
                    assert!(resp.is_ok());
                }
                KVOp::Delete(k) => {
                    let resp = $cli
                        .delete(DeleteRequest::new(KeyRange::key(k.clone())))
                        .await;
                    assert!(resp.is_ok());
                }
            }
        }
    };
}

macro_rules! assert_ops_events {
    ($ops:expr, $stream:expr) => {
        let events = {
            let mut events = vec![];

            loop {
                match tokio::time::timeout(std::time::Duration::from_secs(1), $stream.inbound())
                    .await
                {
                    Ok(etcd_rs::WatchInbound::Ready(resp)) => {
                        for e in resp.events {
                            events.push(match e.event_type {
                                EventType::Put => KVOp::Put(
                                    e.kv.key_str().to_owned(),
                                    e.kv.value_str().to_owned(),
                                ),
                                EventType::Delete => KVOp::Delete(e.kv.key_str().to_owned()),
                            });
                        }
                    }
                    Ok(etcd_rs::WatchInbound::Closed) => break,
                    _ => unreachable!(),
                }
            }
            events
        };

        assert_eq!($ops, events);
    };
}
