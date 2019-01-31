use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

pub struct DeleteRequest {
    key: String,
    end_key: Option<Vec<u8>>,
    prev_kv: bool,
}

impl DeleteRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            end_key: None,
            prev_kv: false,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            end_key: Some(end_key.into().into_bytes()),
            prev_kv: false,
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
                    end = end[0..i + 1].to_vec();
                    break;
                }
            }

            end
        };
        Self {
            key: key,
            end_key: Some(end_key),
            prev_kv: false,
        }
    }

    pub fn with_prev_kv(mut self) -> Self {
        self.prev_kv = true;
        self
    }
}

impl Into<rpc::DeleteRangeRequest> for DeleteRequest {
    fn into(self) -> rpc::DeleteRangeRequest {
        let mut req = rpc::DeleteRangeRequest::new();

        req.set_key(Vec::from(self.key.as_bytes()));
        req.set_prev_kv(self.prev_kv);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }
        req
    }
}

pub struct DeleteResponse {
    resp: rpc::DeleteRangeResponse,
}

impl DeleteResponse {
    pub fn prev_kvs(&self) -> Vec<KeyValue> {
        // FIXME perf
        self.resp
            .get_prev_kvs()
            .iter()
            .map(|kv| From::from(kv.clone()))
            .collect()
    }

    pub fn deleted(&self) -> i64 {
        self.resp.get_deleted()
    }

    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::DeleteRangeResponse> for DeleteResponse {
    fn from(resp: rpc::DeleteRangeResponse) -> Self {
        Self { resp }
    }
}
