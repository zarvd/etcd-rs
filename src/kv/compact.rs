use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug, Clone)]
pub struct CompactRequest {
    proto: etcdserverpb::CompactionRequest,
}

impl CompactRequest {
    pub fn new(revision: i64) -> Self {
        Self {
            proto: etcdserverpb::CompactionRequest {
                revision,
                physical: false,
            },
        }
    }

    pub fn physical(mut self) -> Self {
        self.proto.physical = true;
        self
    }
}

impl From<CompactRequest> for etcdserverpb::CompactionRequest {
    fn from(req: CompactRequest) -> Self {
        req.proto
    }
}

impl From<i64> for CompactRequest {
    fn from(revision: i64) -> Self {
        Self::new(revision)
    }
}

#[derive(Debug, Clone)]
pub struct CompactResponse {
    pub header: ResponseHeader,
}

impl From<etcdserverpb::CompactionResponse> for CompactResponse {
    fn from(proto: etcdserverpb::CompactionResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
        }
    }
}
