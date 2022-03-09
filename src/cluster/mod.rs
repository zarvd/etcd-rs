mod member_add;
mod member_list;
mod member_remove;
mod member_update;

pub use member_add::{MemberAddRequest, MemberAddResponse};
pub use member_list::{MemberListRequest, MemberListResponse};
pub use member_remove::{MemberRemoveRequest, MemberRemoveResponse};
pub use member_update::{MemberUpdateRequest, MemberUpdateResponse};

use async_trait::async_trait;

use crate::proto::etcdserverpb;
use crate::Result;

#[async_trait]
pub trait ClusterOp {
    async fn member_add<R>(&self, req: R) -> Result<MemberAddResponse>
    where
        R: Into<MemberAddRequest> + Send;

    async fn member_remove<R>(&self, req: R) -> Result<MemberRemoveResponse>
    where
        R: Into<MemberRemoveRequest> + Send;

    async fn member_update<R>(&self, req: R) -> Result<MemberUpdateResponse>
    where
        R: Into<MemberUpdateRequest> + Send;

    async fn member_list(&self) -> Result<MemberListResponse>;
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: u64,
    pub name: String,
    pub peer_urls: Vec<String>,
    pub client_urls: Vec<String>,
    pub is_learner: bool,
}

impl From<etcdserverpb::Member> for Member {
    fn from(proto: etcdserverpb::Member) -> Self {
        Self {
            id: proto.id,
            name: proto.name,
            peer_urls: proto.peer_ur_ls,
            client_urls: proto.client_ur_ls,
            is_learner: proto.is_learner,
        }
    }
}

impl Into<etcdserverpb::Member> for Member {
    fn into(self) -> etcdserverpb::Member {
        etcdserverpb::Member {
            id: self.id,
            name: self.name,
            peer_ur_ls: self.peer_urls,
            client_ur_ls: self.client_urls,
            is_learner: self.is_learner,
        }
    }
}
