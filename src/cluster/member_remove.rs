use crate::proto::etcdserverpb;
use crate::{Member, ResponseHeader};

#[derive(Debug, Clone)]
pub struct MemberRemoveRequest {
    proto: etcdserverpb::MemberRemoveRequest,
}

impl MemberRemoveRequest {
    pub fn new(member_id: u64) -> Self {
        Self {
            proto: etcdserverpb::MemberRemoveRequest { id: member_id },
        }
    }
}

impl From<MemberRemoveRequest> for etcdserverpb::MemberRemoveRequest {
    fn from(req: MemberRemoveRequest) -> Self {
        req.proto
    }
}

impl From<u64> for MemberRemoveRequest {
    fn from(id: u64) -> Self {
        Self::new(id)
    }
}

#[derive(Debug, Clone)]
pub struct MemberRemoveResponse {
    pub header: ResponseHeader,
    pub members: Vec<Member>,
}

impl From<etcdserverpb::MemberRemoveResponse> for MemberRemoveResponse {
    fn from(proto: etcdserverpb::MemberRemoveResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            members: proto.members.into_iter().map(From::from).collect(),
        }
    }
}
