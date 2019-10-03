use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Clone, Debug)]
pub struct DeleteRequest {
    key: Vec<u8>,
    end_key: Option<Vec<u8>>,
    prev_kv: bool,
}

impl DeleteRequest {
    pub fn key<N>(key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        DeleteRequest {
            key: key.into(),
            end_key: None,
            prev_kv: false,
        }
    }

    pub fn range<N>(key: N, end_key: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        DeleteRequest {
            key: key.into(),
            end_key: Some(end_key.into()),
            prev_kv: false,
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

        DeleteRequest {
            key,
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

        req.set_key(Vec::from(self.key));
        req.set_prev_kv(self.prev_kv);
        if let Some(range_end) = self.end_key {
            req.set_range_end(range_end);
        }

        req
    }
}

#[derive(Clone, Debug)]
pub struct DeleteResponse {
    header: ResponseHeader,
    deleted: i64,
    prev_kvs: Vec<KeyValue>,
}

impl DeleteResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn deleted(&self) -> i64 {
        self.deleted
    }

    pub fn prev_kvs(&self) -> &[KeyValue] {
        &self.prev_kvs
    }

    pub fn into_prev_kvs(self) -> Vec<KeyValue> {
        self.prev_kvs
    }
}

impl From<rpc::DeleteRangeResponse> for DeleteResponse {
    fn from(mut resp: rpc::DeleteRangeResponse) -> Self {
        DeleteResponse {
            header: resp.take_header().into(),
            deleted: resp.deleted,
            prev_kvs: resp
                .prev_kvs
                .into_vec()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
