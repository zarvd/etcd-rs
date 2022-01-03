use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

pub struct EtcdCluster {
    nodes: HashMap<String, String>,
}

#[allow(dead_code)]
impl EtcdCluster {
    pub fn new(with_tls: bool) -> Self {
        Command::new("make")
            .env("ETCD_CLUSTER_WITH_TLS", with_tls.to_string())
            .arg("setup-etcd-cluster")
            .output()
            .expect("setup-etcd-cluster");

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

    pub fn endpoints(&self) -> Vec<String> {
        self.nodes.values().cloned().collect()
    }

    pub fn start_node(&self, i: u64) {
        Command::new("make")
            .env("ETCD_NODE", format!("etcd-{}", i))
            .arg("start-etcd-node")
            .output()
            .expect("start etcd node");
        std::thread::sleep(Duration::from_secs(3));
    }

    pub fn stop_node(&self, i: u64) {
        Command::new("make")
            .env("ETCD_NODE", format!("etcd-{}", i))
            .arg("stop-etcd-node")
            .output()
            .expect("stop etcd node");
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

#[allow(dead_code)]
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

    pub fn stop_etcd_node() {}

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
