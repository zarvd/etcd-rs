use crate::proto::etcdserverpb;
use crate::{Member, ResponseHeader};

#[derive(Debug, Clone)]
pub struct MemberAddRequest {
    proto: etcdserverpb::MemberAddRequest,
}

impl MemberAddRequest {
    pub fn new(peer_urls: impl Into<Vec<String>>, is_learner: bool) -> Self {
        Self {
            proto: etcdserverpb::MemberAddRequest {
                peer_ur_ls: peer_urls.into(),
                is_learner,
            },
        }
    }
}

impl<I> From<I> for MemberAddRequest
where
    I: Into<Vec<String>>,
{
    fn from(peer_urls: I) -> Self {
        Self::new(peer_urls, false)
    }
}

impl From<MemberAddRequest> for etcdserverpb::MemberAddRequest {
    fn from(req: MemberAddRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct MemberAddResponse {
    pub header: ResponseHeader,
    pub member: Member,
    pub members: Vec<Member>,
}

impl From<etcdserverpb::MemberAddResponse> for MemberAddResponse {
    fn from(proto: etcdserverpb::MemberAddResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            member: From::from(proto.member.expect("must get a new member")),
            members: proto.members.into_iter().map(From::from).collect(),
        }
    }
}
