use crate::kv::{DeleteRequest, DeleteResponse, GetRequest, GetResponse, PutRequest, PutResponse};
use crate::proto::rpc;
use crate::ResponseHeader;

#[derive(Debug)]
pub enum TxnCmp {
    Equal,
    NotEqual,
    Greater,
    Less,
}

impl Into<rpc::Compare_CompareResult> for TxnCmp {
    fn into(self) -> rpc::Compare_CompareResult {
        match self {
            TxnCmp::Equal => rpc::Compare_CompareResult::EQUAL,
            TxnCmp::NotEqual => rpc::Compare_CompareResult::NOT_EQUAL,
            TxnCmp::Greater => rpc::Compare_CompareResult::GREATER,
            TxnCmp::Less => rpc::Compare_CompareResult::LESS,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TxnRequest {
    compare: Vec<rpc::Compare>,
    success: Vec<rpc::RequestOp>,
    failure: Vec<rpc::RequestOp>,
}

impl TxnRequest {
    pub fn new() -> Self {
        Self {
            compare: Default::default(),
            success: Default::default(),
            failure: Default::default(),
        }
    }

    pub fn when_value<N>(mut self, target: N, cmp: TxnCmp, value: N) -> Self
    where
        N: Into<String>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::VALUE);
        compare.set_key(target.into().into_bytes());
        compare.set_result(cmp.into());
        compare.set_value(value.into().into_bytes());

        self.compare.push(compare);
        self
    }

    pub fn when_version<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<String>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::VERSION);
        compare.set_key(target.into().into_bytes());
        compare.set_result(cmp.into());
        compare.set_version(value);

        self.compare.push(compare);
        self
    }

    pub fn when_create_revision<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<String>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::CREATE);
        compare.set_key(target.into().into_bytes());
        compare.set_result(cmp.into());
        compare.set_create_revision(value);

        self.compare.push(compare);
        self
    }

    pub fn when_mod_revision<N>(mut self, target: N, cmp: TxnCmp, value: i64) -> Self
    where
        N: Into<String>,
    {
        let mut compare = rpc::Compare::new();
        compare.set_target(rpc::Compare_CompareTarget::CREATE);
        compare.set_key(target.into().into_bytes());
        compare.set_result(cmp.into());
        compare.set_mod_revision(value);

        self.compare.push(compare);
        self
    }

    // TODO
    // pub fn when_lease(target: String, cmp: TxnCmp, value: i64) -> Self {
    //     Self {
    //         target: TxnTarget::Lease(target),
    //         value: TxnTargetValue::Lease(value),
    //         compare,
    //     }
    // }

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

        req.set_compare(From::from(self.compare));
        req.set_success(From::from(self.success));
        req.set_failure(From::from(self.failure));

        req
    }
}

#[derive(Debug)]
pub enum TxnResult {
    Get(GetResponse),
    Put(PutResponse),
    Delete(DeleteResponse),
    Txn(TxnResponse),
}

#[derive(Debug)]
pub struct TxnResponse {
    resp: rpc::TxnResponse,
}

impl TxnResponse {
    pub fn header(&self) -> ResponseHeader {
        // FIXME perf
        From::from(self.resp.get_header().clone())
    }

    pub fn is_succeeded(&self) -> bool {
        self.resp.get_succeeded()
    }

    pub fn results(&self) -> Vec<TxnResult> {
        //FIXME perf
        self.resp
            .get_responses()
            .iter()
            .map(|resp| match &resp.response {
                Some(rpc::ResponseOp_oneof_response::response_range(resp)) => {
                    TxnResult::Get(From::from(resp.clone()))
                }
                Some(rpc::ResponseOp_oneof_response::response_put(resp)) => {
                    TxnResult::Put(From::from(resp.clone()))
                }
                Some(rpc::ResponseOp_oneof_response::response_delete_range(resp)) => {
                    TxnResult::Delete(From::from(resp.clone()))
                }
                Some(rpc::ResponseOp_oneof_response::response_txn(resp)) => {
                    TxnResult::Txn(From::from(resp.clone()))
                }
                None => panic!("failed to fetch transaction response"),
            })
            .collect()
    }
}

impl From<rpc::TxnResponse> for TxnResponse {
    fn from(resp: rpc::TxnResponse) -> Self {
        Self { resp }
    }
}
