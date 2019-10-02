use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

pub struct GetRequest {
    key: Vec<u8>,
    end_key: Option<Vec<u8>>,
    limit: i64,
    revision: i64,
    serializable: bool,
    keys_only: bool,
    count_only: bool,
}

impl GetRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            end_key: None,
            limit: 0,
            revision: 0,
            serializable: false,
            keys_only: false,
            count_only: false,
        }
    }

    pub fn prefix<N>(prefix: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let key = prefix.into();
	    let end_key = {
		    let mut end = key.clone();
		    let last = end
			    .last_mut()
			    .copied()
			    .unwrap_or(0);

		    if last == std::u8::MAX {
			    end.push(1);
		    } else {
			    last += 1;
		    }

            end
        };
        Self {
            key,
            end_key: Some(end_key),
            limit: 0,
            revision: 0,
            serializable: false,
            keys_only: false,
            count_only: false,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        Self {
            key: key.into(),
            end_key: Some(end_key.into()),
            limit: 0,
            revision: 0,
            serializable: false,
            keys_only: false,
            count_only: false,
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
}

impl Into<rpc::RangeRequest> for GetRequest {
    fn into(self) -> rpc::RangeRequest {
        let mut req = rpc::RangeRequest::new();

        req.set_key(self.key);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }

        req.set_limit(self.limit);
        req.set_revision(self.revision);
        req.set_keys_only(self.keys_only);
        req.set_count_only(self.count_only);
        req.set_serializable(self.serializable);

        req
    }
}

#[derive(Debug)]
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
}

impl From<rpc::RangeResponse> for GetResponse {
    fn from(mut resp: rpc::RangeResponse) -> Self {
	    GetResponse {
		    header: resp.take_header().into(),
		    more: resp.more,
		    count: resp.count,
		    kvs: resp
			    .kvs
			    .into_vec()
			    .into_iter()
			    .map(|kv| kv.into())
			    .collect(),
	    }
    }
}
