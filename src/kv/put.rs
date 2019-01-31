use crate::kv::KeyValue;
use crate::proto::rpc;
use crate::ResponseHeader;

pub struct PutRequest {
    key: String,
    value: String,
    lease: Option<i64>,
    prev_kv: bool,
    ignore_value: bool,
    ignore_lease: bool,
}

impl PutRequest {
    pub fn new<N>(key: N, value: N) -> Self
    where
        N: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
            lease: None,
            prev_kv: false,
            ignore_lease: false,
            ignore_value: false,
        }
    }

    pub fn with_lease(mut self, lease_id: i64) -> Self {
        self.lease = Some(lease_id);
        self
    }

    pub fn with_prev_kv(mut self) -> Self {
        self.prev_kv = true;
        self
    }

    pub fn with_ignore_value(mut self) -> Self {
        self.ignore_value = true;
        self
    }

    pub fn with_ignore_lease(mut self) -> Self {
        self.ignore_lease = true;
        self
    }
}

impl Into<rpc::PutRequest> for PutRequest {
    fn into(self) -> rpc::PutRequest {
        let mut req = rpc::PutRequest::new();

        req.set_key(self.key.into_bytes());
        req.set_value(self.value.into_bytes());
        req.set_ignore_lease(self.ignore_lease);
        req.set_ignore_value(self.ignore_value);
        req.set_prev_kv(self.prev_kv);
        if let Some(lease) = self.lease {
            req.set_lease(lease);
        }

        req
    }
}

pub struct PutResponse {
    resp: rpc::PutResponse,
}

impl PutResponse {
    pub fn prev_kv(&self) -> KeyValue {
        // FIXME perf
        From::from(self.resp.get_prev_kv().clone())
    }

    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }
}

impl From<rpc::PutResponse> for PutResponse {
    fn from(resp: rpc::PutResponse) -> Self {
        Self { resp }
    }
}
