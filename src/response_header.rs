use crate::proto::etcdserverpb;

#[derive(Debug, Clone)]
pub struct ResponseHeader {
    proto: crate::proto::etcdserverpb::ResponseHeader,
}

impl ResponseHeader {
    /// Get the ID of the cluster which sent the response.
    pub fn cluster_id(&self) -> u64 {
        self.proto.cluster_id
    }

    /// Get the ID of the member which sent the response.
    pub fn member_id(&self) -> u64 {
        self.proto.member_id
    }

    /// Get the key-value store revision when the request was applied.
    pub fn revision(&self) -> i64 {
        self.proto.revision
    }

    /// Get the raft term when the request was applied.
    pub fn raft_term(&self) -> u64 {
        self.proto.raft_term
    }
}

impl From<etcdserverpb::ResponseHeader> for ResponseHeader {
    fn from(proto: etcdserverpb::ResponseHeader) -> Self {
        Self { proto }
    }
}
