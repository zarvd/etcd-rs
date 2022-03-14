use crate::proto::etcdserverpb;
use crate::{Member, ResponseHeader};

#[derive(Debug, Clone)]
pub struct MemberUpdateRequest {
    proto: etcdserverpb::MemberUpdateRequest,
}

impl MemberUpdateRequest {
    pub fn new(member_id: u64, peer_urls: impl Into<Vec<String>>) -> Self {
        Self {
            proto: etcdserverpb::MemberUpdateRequest {
                id: member_id,
                peer_ur_ls: peer_urls.into(),
            },
        }
    }
}

impl<I> From<(u64, I)> for MemberUpdateRequest
where
    I: Into<Vec<String>>,
{
    fn from((id, peer_urls): (u64, I)) -> Self {
        Self::new(id, peer_urls)
    }
}

impl From<MemberUpdateRequest> for etcdserverpb::MemberUpdateRequest {
    fn from(req: MemberUpdateRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct MemberUpdateResponse {
    pub header: ResponseHeader,
    pub members: Vec<Member>,
}

impl From<etcdserverpb::MemberUpdateResponse> for MemberUpdateResponse {
    fn from(proto: etcdserverpb::MemberUpdateResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            members: proto.members.into_iter().map(From::from).collect(),
        }
    }
}
