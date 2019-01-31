use crate::proto::rpc;

#[derive(Debug)]
pub struct ResponseHeader {
    header: rpc::ResponseHeader,
}

impl ResponseHeader {
    pub fn cluster_id(&self) -> u64 {
        self.header.get_cluster_id()
    }

    pub fn member_id(&self) -> u64 {
        self.header.get_member_id()
    }

    pub fn revision(&self) -> i64 {
        self.header.get_revision()
    }

    pub fn raft_term(&self) -> u64 {
        self.header.get_raft_term()
    }
}

impl From<rpc::ResponseHeader> for ResponseHeader {
    fn from(header: rpc::ResponseHeader) -> Self {
        Self { header }
    }
}
