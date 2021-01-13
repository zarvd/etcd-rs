pbwrap_response!(ResponseHeader);

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
    pub fn revision(&self) -> u64 {
        self.proto.revision as u64
    }

    /// Get the raft term when the request was applied.
    pub fn raft_term(&self) -> u64 {
        self.proto.raft_term
    }
}
