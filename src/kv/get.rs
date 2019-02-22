use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

pub struct GetRequest {
    key: String,
    end_key: Option<Vec<u8>>,
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
            serializable: false,
            keys_only: false,
            count_only: false,
        }
    }

    pub fn prefix<N>(prefix: N) -> Self
    where
        N: Into<String>,
    {
        let key = prefix.into();
        let end_key = {
            let mut end = key.clone().into_bytes();

            for i in (0..end.len()).rev() {
                if end[i] < 0xff {
                    end[i] += 1;
                    end = end[0..=i].to_vec();
                    break;
                }
            }

            end
        };
        Self {
            key: key,
            end_key: Some(end_key),
            serializable: false,
            keys_only: false,
            count_only: false,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            end_key: Some(end_key.into().into_bytes()),
            serializable: false,
            keys_only: false,
            count_only: false,
        }
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

        req.set_key(self.key.into_bytes());
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }
        req.set_keys_only(self.keys_only);
        req.set_count_only(self.count_only);
        req.set_serializable(self.serializable);

        req
    }
}

#[derive(Debug)]
pub struct GetResponse {
    resp: rpc::RangeResponse,
}

impl GetResponse {
    pub fn kvs(&self) -> Vec<KeyValue> {
        // FIXME perf
        self.resp
            .get_kvs()
            .iter()
            .map(|kv| From::from(kv.clone()))
            .collect()
    }

    pub fn has_more(&self) -> bool {
        self.resp.get_more()
    }

    pub fn count(&self) -> i64 {
        self.resp.get_count()
    }

    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::RangeResponse> for GetResponse {
    fn from(resp: rpc::RangeResponse) -> Self {
        Self { resp }
    }
}
