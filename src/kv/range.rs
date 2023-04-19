use super::{KeyRange, KeyValue};
use crate::proto::etcdserverpb;
use crate::ResponseHeader;

#[derive(Debug)]
pub struct RangeRequest {
    proto: etcdserverpb::RangeRequest,
}

impl RangeRequest {
    /// Creates a new RangeRequest for the specified key range.
    pub fn new(key_range: KeyRange) -> Self {
        Self {
            proto: etcdserverpb::RangeRequest {
                key: key_range.key,
                range_end: key_range.range_end,
                limit: 0,
                revision: 0,
                sort_order: 0,
                sort_target: 0,
                serializable: false,
                keys_only: false,
                count_only: false,
                min_mod_revision: 0,
                max_mod_revision: 0,
                min_create_revision: 0,
                max_create_revision: 0,
            },
        }
    }

    /// Sets the maximum number of keys returned for the request.
    /// When limit is set to 0, it is treated as no limit.
    pub fn limit(mut self, limit: u64) -> Self {
        self.proto.limit = limit as i64;
        self
    }

    pub fn revision(mut self, revision: i64) -> Self {
        self.proto.revision = revision;
        self
    }

    pub fn sort_by_key(mut self, order: SortOrder) -> Self {
        self.proto.sort_target = etcdserverpb::range_request::SortTarget::Key as i32;
        self.proto.sort_order = order.into();
        self
    }

    pub fn sort_by_version(mut self, order: SortOrder) -> Self {
        self.proto.sort_target = etcdserverpb::range_request::SortTarget::Version as i32;
        self.proto.sort_order = order.into();
        self
    }
}

impl<T> From<T> for RangeRequest
where
    T: Into<KeyRange>,
{
    fn from(key_range: T) -> Self {
        Self::new(key_range.into())
    }
}

impl From<RangeRequest> for etcdserverpb::RangeRequest {
    fn from(x: RangeRequest) -> Self {
        x.proto
    }
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl From<SortOrder> for etcdserverpb::range_request::SortOrder {
    fn from(value: SortOrder) -> Self {
        match value {
            SortOrder::Ascending => etcdserverpb::range_request::SortOrder::Ascend,
            SortOrder::Descending => etcdserverpb::range_request::SortOrder::Descend,
        }
    }
}

impl From<SortOrder> for i32 {
    fn from(value: SortOrder) -> Self {
        let order: etcdserverpb::range_request::SortOrder = value.into();
        order as i32
    }
}

#[derive(Debug, Clone)]
pub struct RangeResponse {
    pub header: ResponseHeader,
    pub kvs: Vec<KeyValue>,
    pub has_more: bool,
    pub count: u64,
}

impl From<etcdserverpb::RangeResponse> for RangeResponse {
    fn from(proto: etcdserverpb::RangeResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            kvs: proto.kvs.into_iter().map(From::from).collect(),
            has_more: proto.more,
            count: proto.count as u64,
        }
    }
}
