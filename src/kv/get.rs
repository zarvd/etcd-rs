use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct GetRequest {
    key: Vec<u8>,
    end_key: Option<Vec<u8>>,
    limit: i64,
    revision: i64,
    sort_order: rpc::RangeRequest_SortOrder,
    sort_target: rpc::RangeRequest_SortTarget,
    serializable: bool,
    keys_only: bool,
    count_only: bool,
    min_mod_revision: i64,
    max_mod_revision: i64,
    min_create_revision: i64,
    max_create_revision: i64,
}

impl GetRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        GetRequest {
            key: key.into(),
            end_key: None,
            limit: 0,
            revision: 0,
            sort_order: Default::default(),
            sort_target: Default::default(),
            serializable: false,
            keys_only: false,
            count_only: false,
            min_mod_revision: 0,
            max_mod_revision: 0,
            min_create_revision: 0,
            max_create_revision: 0,
        }
    }

    pub fn prefix<N>(prefix: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let key = prefix.into();
        let end_key = {
            let mut end = key.clone();
            let last = end.last().copied().unwrap_or(0);

            if last == std::u8::MAX {
                end.push(1);
            } else {
                *end.last_mut().unwrap() += 1;
            }

            end
        };

        GetRequest {
            key,
            end_key: Some(end_key),
            limit: 0,
            revision: 0,
            sort_order: Default::default(),
            sort_target: Default::default(),
            serializable: false,
            keys_only: false,
            count_only: false,
            min_mod_revision: 0,
            max_mod_revision: 0,
            min_create_revision: 0,
            max_create_revision: 0,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        GetRequest {
            key: key.into(),
            end_key: Some(end_key.into()),
            limit: 0,
            revision: 0,
            sort_order: Default::default(),
            sort_target: Default::default(),
            serializable: false,
            keys_only: false,
            count_only: false,
            min_mod_revision: 0,
            max_mod_revision: 0,
            min_create_revision: 0,
            max_create_revision: 0,
        }
    }

    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_revision(mut self, revision: i64) -> Self {
        self.revision = revision;
        self
    }

    pub fn with_order_ascend(mut self) -> Self {
        self.sort_order = rpc::RangeRequest_SortOrder::ASCEND;
        self
    }

    pub fn with_order_descend(mut self) -> Self {
        self.sort_order = rpc::RangeRequest_SortOrder::DESCEND;
        self
    }

    pub fn with_sort_key(mut self) -> Self {
        self.sort_target = rpc::RangeRequest_SortTarget::KEY;
        self
    }

    pub fn with_sort_version(mut self) -> Self {
        self.sort_target = rpc::RangeRequest_SortTarget::VERSION;
        self
    }

    pub fn with_sort_create_revision(mut self) -> Self {
        self.sort_target = rpc::RangeRequest_SortTarget::CREATE;
        self
    }

    pub fn with_sort_mod_revision(mut self) -> Self {
        self.sort_target = rpc::RangeRequest_SortTarget::MOD;
        self
    }

    pub fn with_sort_value(mut self) -> Self {
        self.sort_target = rpc::RangeRequest_SortTarget::VALUE;
        self
    }

    pub fn with_serializable(mut self) -> Self {
        self.serializable = true;
        self
    }

    pub fn with_count_only(mut self) -> Self {
        self.count_only = true;
        self
    }

    pub fn with_keys_only(mut self) -> Self {
        self.keys_only = true;
        self
    }

    pub fn with_min_mod_revision(mut self, mod_revision: i64) -> Self {
        self.min_mod_revision = mod_revision;
        self
    }

    pub fn with_max_mod_revision(mut self, mod_revision: i64) -> Self {
        self.max_mod_revision = mod_revision;
        self
    }

    pub fn with_min_create_revision(mut self, create_revision: i64) -> Self {
        self.min_create_revision = create_revision;
        self
    }

    pub fn with_max_create_revision(mut self, create_revision: i64) -> Self {
        self.max_create_revision = create_revision;
        self
    }
}

impl Into<rpc::RangeRequest> for GetRequest {
    fn into(self) -> rpc::RangeRequest {
        let mut req = rpc::RangeRequest::new();

        req.set_key(self.key);
        req.set_limit(self.limit);
        req.set_revision(self.revision);
        req.set_sort_order(self.sort_order);
        req.set_sort_target(self.sort_target);
        req.set_serializable(self.serializable);
        req.set_keys_only(self.keys_only);
        req.set_count_only(self.count_only);
        req.set_min_mod_revision(self.min_mod_revision);
        req.set_max_mod_revision(self.max_mod_revision);
        req.set_min_create_revision(self.min_create_revision);
        req.set_max_create_revision(self.max_create_revision);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }

        req
    }
}

#[derive(Clone, Debug)]
pub struct GetResponse {
    header: ResponseHeader,
    more: bool,
    count: i64,
    kvs: Vec<KeyValue>,
}

impl GetResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn has_more(&self) -> bool {
        self.more
    }

    pub fn count(&self) -> i64 {
        self.count
    }

    pub fn kvs(&self) -> &[KeyValue] {
        &self.kvs
    }

    pub fn into_kvs(self) -> Vec<KeyValue> {
        self.kvs
    }
}

impl From<rpc::RangeResponse> for GetResponse {
    fn from(mut resp: rpc::RangeResponse) -> Self {
        GetResponse {
            header: resp.take_header().into(),
            more: resp.more,
            count: resp.count,
            kvs: resp.kvs.into_vec().into_iter().map(Into::into).collect(),
        }
    }
}
