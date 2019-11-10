use std::time::Duration;

use crate::proto::etcdserverpb;

pub struct LeaseGrantRequest {
    proto: etcdserverpb::LeaseGrantRequest,
}

impl LeaseGrantRequest {
    pub fn new(ttl: Duration) -> Self {
        let proto = etcdserverpb::LeaseGrantRequest {
            ttl: ttl.as_secs() as i64,
            id: 0,
        };

        Self { proto }
    }

    /// set custom lease ID
    pub fn set_id(&mut self, id: u64) {
        self.proto.id = id as i64
    }
}

impl Into<etcdserverpb::LeaseGrantRequest> for LeaseGrantRequest {
    fn into(self) -> etcdserverpb::LeaseGrantRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct LeaseGrantResponse {
    proto: etcdserverpb::LeaseGrantResponse,
}

impl LeaseGrantResponse {
    pub fn id(&self) -> u64 {
        self.proto.id as u64
    }
}

impl From<etcdserverpb::LeaseGrantResponse> for LeaseGrantResponse {
    fn from(resp: etcdserverpb::LeaseGrantResponse) -> Self {
        Self { proto: resp }
    }
}
