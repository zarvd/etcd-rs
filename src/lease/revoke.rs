use crate::proto::etcdserverpb;

pub struct LeaseRevokeRequest {
    proto: etcdserverpb::LeaseRevokeRequest,
}

impl LeaseRevokeRequest {
    pub fn new(id: u64) -> Self {
        let proto = etcdserverpb::LeaseRevokeRequest { id: id as i64 };

        Self { proto }
    }
}

impl Into<etcdserverpb::LeaseRevokeRequest> for LeaseRevokeRequest {
    fn into(self) -> etcdserverpb::LeaseRevokeRequest {
        self.proto
    }
}

#[derive(Debug)]
pub struct LeaseRevokeResponse {
    proto: etcdserverpb::LeaseRevokeResponse,
}

impl From<etcdserverpb::LeaseRevokeResponse> for LeaseRevokeResponse {
    fn from(resp: etcdserverpb::LeaseRevokeResponse) -> Self {
        Self { proto: resp }
    }
}
