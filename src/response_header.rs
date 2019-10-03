use crate::proto::rpc;

#[derive(Clone, Debug)]
pub struct ResponseHeader {
    cluster_id: u64,
    member_id: u64,
    revision: i64,
    raft_term: u64,
}

impl ResponseHeader {
    pub fn cluster_id(&self) -> u64 {
        self.cluster_id
    }

    pub fn member_id(&self) -> u64 {
        self.member_id
    }

    pub fn revision(&self) -> i64 {
        self.revision
    }

    pub fn raft_term(&self) -> u64 {
        self.raft_term
    }
}

impl From<rpc::ResponseHeader> for ResponseHeader {
    fn from(header: rpc::ResponseHeader) -> Self {
        Self {
            cluster_id: header.cluster_id,
            member_id: header.member_id,
            revision: header.revision,
            raft_term: header.raft_term,
        }
    }
}
