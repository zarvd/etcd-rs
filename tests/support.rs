#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;
use std::process::Command;

pub struct EtcdCluster {
    nodes: HashMap<String, String>,
}

impl EtcdCluster {
    pub fn new(with_tls: bool) -> Self {
        println!("etcd cluster starting");
        assert!(Command::new("make")
            .env("ETCD_CLUSTER_WITH_TLS", with_tls.to_string())
            .arg("setup-etcd-cluster")
            .output()
            .expect("setup-etcd-cluster")
            .status
            .success());
        println!("etcd cluster started");

        let nodes: HashMap<_, _> = (1..=3)
            .map(|i| {
                let scheme = if with_tls { "https" } else { "http" };

                (
                    format!("etcd-{}", i),
                    format!("{}://127.0.0.1:{}2379", scheme, i),
                )
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

    pub fn endpoints(&self) -> Vec<String> {
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
        Command::new("make")
            .arg("teardown-etcd-cluster")
            .output()
            .expect("teardown etcd cluster");
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

        Client::connect(ClientConfig {
            endpoints: self.etcd_cluster.endpoints(),
            auth: self.auth.clone(),
            tls: None,
        })
        .await
        .expect("connect to etcd cluster")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KVOp {
    Put(String, String),
    Delete(String),
}

macro_rules! assert_watch_created {
    ($tunnel:expr) => {
        if let etcd_rs::WatchInbound::Ready(mut resp) = $tunnel.inbound().next().await.unwrap() {
            assert!(resp.created());
            assert!(resp.take_events().is_empty());
        } else {
            unreachable!();
        }
    };
}

macro_rules! apply_kv_ops {
    ($cli:expr, $ops:expr) => {
        for op in $ops.iter() {
            match op {
                KVOp::Put(k, v) => {
                    let resp = $cli.kv().put(PutRequest::new(k.clone(), v.clone())).await;
                    assert!(resp.is_ok());
                }
                KVOp::Delete(k) => {
                    let resp = $cli
                        .kv()
                        .delete(DeleteRequest::new(KeyRange::key(k.clone())))
                        .await;
                    assert!(resp.is_ok());
                }
            }
        }
    };
}

macro_rules! assert_ops_events {
    ($ops:expr, $tunnel:expr) => {
        let events = {
            let mut events = vec![];

            while let Ok(incoming) = timeout(Duration::from_secs(1), $tunnel.inbound().next()).await
            {
                if let Some(etcd_rs::WatchInbound::Ready(mut resp)) = incoming {
                    for mut e in resp.take_events() {
                        let kv = e.take_kv().unwrap();
                        events.push(match e.event_type() {
                            EventType::Put => {
                                KVOp::Put(kv.key_str().to_owned(), kv.value_str().to_owned())
                            }
                            EventType::Delete => KVOp::Delete(kv.key_str().to_owned()),
                        });
                    }
                } else {
                    unreachable!();
                }
            }

            events
        };

        assert_eq!($ops, events);
    };
}
