use crate::kv::{DeleteRequest, DeleteResponse, GetRequest, GetResponse, PutRequest, PutResponse};
use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Copy, Clone, Debug)]
pub enum TxnCmp {
    Equal,
    Greater,
    Less,
    NotEqual,
}

impl Into<rpc::Compare_CompareResult> for TxnCmp {
    fn into(self) -> rpc::Compare_CompareResult {
        match self {
            TxnCmp::Equal => rpc::Compare_CompareResult::EQUAL,
            TxnCmp::Greater => rpc::Compare_CompareResult::GREATER,
            TxnCmp::Less => rpc::Compare_CompareResult::LESS,
            TxnCmp::NotEqual => rpc::Compare_CompareResult::NOT_EQUAL,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TxnOp {
    Get(GetRequest),
    Put(PutRequest),
    Delete(DeleteRequest),
    Txn(TxnRequest),
}

impl From<GetRequest> for TxnOp {
    fn from(req: GetRequest) -> Self {
        TxnOp::Get(req)
    }
}

impl From<PutRequest> for TxnOp {
    fn from(req: PutRequest) -> Self {
        TxnOp::Put(req)
    }
}

impl From<DeleteRequest> for TxnOp {
    fn from(req: DeleteRequest) -> Self {
        TxnOp::Delete(req)
    }
}

impl From<TxnRequest> for TxnOp {
    fn from(req: TxnRequest) -> Self {
        TxnOp::Txn(req)
    }
}

impl Into<rpc::RequestOp> for TxnOp {
    fn into(self) -> rpc::RequestOp {
        let mut op = rpc::RequestOp::new();

        match self {
            TxnOp::Get(req) => op.set_request_range(req.into()),
            TxnOp::Put(req) => op.set_request_put(req.into()),
            TxnOp::Delete(req) => op.set_request_delete_range(req.into()),
            TxnOp::Txn(req) => op.set_request_txn(req.into()),
        }

        op
    }
}

#[derive(Clone, Debug)]
pub struct TxnRequest {
    compare: Vec<rpc::Compare>,
    success: Vec<rpc::RequestOp>,
    failure: Vec<rpc::RequestOp>,
}

impl TxnRequest {
    pub fn new() -> Self {
        TxnRequest {
            compare: Default::default(),
            success: Default::default(),
            failure: Default::default(),
        }
    }

    pub fn when_version<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::VERSION);
        compare.set_key(target.into());
        compare.set_result(cmp.into());
        compare.set_version(value);

        self.compare.push(compare);
        self
    }

    pub fn when_create_revision<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::CREATE);
        compare.set_key(target.into());
        compare.set_result(cmp.into());
        compare.set_create_revision(value);

        self.compare.push(compare);
        self
    }

    pub fn when_mod_revision<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::MOD);
        compare.set_key(target.into());
        compare.set_result(cmp.into());
        compare.set_mod_revision(value);

        self.compare.push(compare);
        self
    }

    pub fn when_value<N>(mut self, target: N, cmp: TxnCmp, value: N) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::VALUE);
        compare.set_key(target.into());
        compare.set_result(cmp.into());
        compare.set_value(value.into());

        self.compare.push(compare);
        self
    }

    pub fn when_lease<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<Vec<u8>>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::LEASE);
        compare.set_key(target.into());
        compare.set_result(cmp.into());
        compare.set_lease(value);

        self.compare.push(compare);
        self
    }

    pub fn and_then<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.success.push(op.into().into());
        self
    }

    pub fn or_else<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.failure.push(op.into().into());
        self
    }
}

impl Into<rpc::TxnRequest> for TxnRequest {
    fn into(self) -> rpc::TxnRequest {
        let mut req = rpc::TxnRequest::new();

        req.set_compare(self.compare.into());
        req.set_success(self.success.into());
        req.set_failure(self.failure.into());

        req
    }
}

#[derive(Clone, Debug)]
pub enum TxnResult {
    Get(GetResponse),
    Put(PutResponse),
    Delete(DeleteResponse),
    Txn(TxnResponse),
}

#[derive(Clone, Debug)]
pub struct TxnResponse {
    header: ResponseHeader,
    succeeded: bool,
    results: Vec<TxnResult>,
}

impl TxnResponse {
    pub fn header(&self) -> &ResponseHeader {
        &self.header
    }

    pub fn is_succeeded(&self) -> bool {
        self.succeeded
    }

    pub fn results(&self) -> &[TxnResult] {
        &self.results
    }
}

impl From<rpc::TxnResponse> for TxnResponse {
    fn from(mut resp: rpc::TxnResponse) -> Self {
        let header = resp.take_header().into();
        let results = resp
            .responses
            .into_vec()
            .into_iter()
            .map(|resp| match resp.response {
                Some(rpc::ResponseOp_oneof_response::response_range(resp)) => {
                    TxnResult::Get(resp.into())
                }
                Some(rpc::ResponseOp_oneof_response::response_put(resp)) => {
                    TxnResult::Put(resp.into())
                }
                Some(rpc::ResponseOp_oneof_response::response_delete_range(resp)) => {
                    TxnResult::Delete(resp.into())
                }
                Some(rpc::ResponseOp_oneof_response::response_txn(resp)) => {
                    TxnResult::Txn(resp.into())
                }
                // FIXME: panic
                None => panic!("failed to fetch transaction response"),
            })
            .collect();

        TxnResponse {
            header,
            succeeded: resp.succeeded,
            results,
        }
    }
}
