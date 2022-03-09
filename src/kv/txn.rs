use super::{
    DeleteRequest, DeleteResponse, KeyRange, PutRequest, PutResponse, RangeRequest, RangeResponse,
};
use crate::proto::etcdserverpb;
use crate::ResponseHeader;
use etcdserverpb::compare::{CompareResult, CompareTarget, TargetUnion};
use etcdserverpb::Compare;

#[derive(Debug)]
pub struct TxnRequest {
    proto: etcdserverpb::TxnRequest,
}

impl TxnRequest {
    /// Creates a new TxnRequest.
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::TxnRequest {
                compare: vec![],
                success: vec![],
                failure: vec![],
            },
        }
    }

    /// Adds a version compare.
    pub fn when_version(mut self, key_range: KeyRange, cmp: TxnCmp, version: usize) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Version as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::Version(version as i64)),
        });
        self
    }

    /// Adds a create revision compare.
    pub fn when_create_revision(
        mut self,
        key_range: KeyRange,
        cmp: TxnCmp,
        revision: usize,
    ) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Create as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::CreateRevision(revision as i64)),
        });
        self
    }

    /// Adds a mod revision compare.
    pub fn when_mod_revision(mut self, key_range: KeyRange, cmp: TxnCmp, revision: usize) -> Self {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Mod as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::ModRevision(revision as i64)),
        });
        self
    }

    /// Adds a value compare.
    pub fn when_value<V>(mut self, key_range: KeyRange, cmp: TxnCmp, value: V) -> Self
    where
        V: Into<Vec<u8>>,
    {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Value as i32,
            key: key_range.key,
            range_end: key_range.range_end,
            target_union: Some(TargetUnion::Value(value.into())),
        });
        self
    }

    /// If compare success, then execute the specified operations.
    pub fn and_then<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.proto.success.push(op.into().into());
        self
    }

    /// If compare fail, then execute the specified operations.
    pub fn or_else<O>(mut self, op: O) -> Self
    where
        O: Into<TxnOp>,
    {
        self.proto.failure.push(op.into().into());
        self
    }
}

impl Default for TxnRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TxnRequest> for crate::proto::etcdserverpb::TxnRequest {
    fn from(x: TxnRequest) -> Self {
        x.proto
    }
}

/// Transaction Operation.
pub enum TxnOp {
    Range(RangeRequest),
    Put(PutRequest),
    Delete(DeleteRequest),
    Txn(TxnRequest),
}

impl From<TxnOp> for etcdserverpb::RequestOp {
    fn from(x: TxnOp) -> etcdserverpb::RequestOp {
        use etcdserverpb::request_op::Request;

        let req = match x {
            TxnOp::Range(req) => Request::RequestRange(req.into()),
            TxnOp::Put(req) => Request::RequestPut(req.into()),
            TxnOp::Delete(req) => Request::RequestDeleteRange(req.into()),
            TxnOp::Txn(req) => Request::RequestTxn(req.into()),
        };

        etcdserverpb::RequestOp { request: Some(req) }
    }
}

impl From<RangeRequest> for TxnOp {
    fn from(req: RangeRequest) -> Self {
        Self::Range(req)
    }
}

impl From<PutRequest> for TxnOp {
    fn from(req: PutRequest) -> Self {
        Self::Put(req)
    }
}

impl From<DeleteRequest> for TxnOp {
    fn from(req: DeleteRequest) -> Self {
        Self::Delete(req)
    }
}

impl From<TxnRequest> for TxnOp {
    fn from(req: TxnRequest) -> Self {
        Self::Txn(req)
    }
}

/// Transaction Comparation.
pub enum TxnCmp {
    Equal,
    NotEqual,
    Greater,
    Less,
}

impl From<TxnCmp> for CompareResult {
    fn from(x: TxnCmp) -> CompareResult {
        match x {
            TxnCmp::Equal => CompareResult::Equal,
            TxnCmp::NotEqual => CompareResult::NotEqual,
            TxnCmp::Greater => CompareResult::Greater,
            TxnCmp::Less => CompareResult::Less,
        }
    }
}

/// Response transaction operation.
#[derive(Debug, Clone)]
pub enum TxnOpResponse {
    Range(RangeResponse),
    Put(PutResponse),
    Delete(DeleteResponse),
    Txn(TxnResponse),
}

impl From<etcdserverpb::ResponseOp> for TxnOpResponse {
    fn from(mut resp: etcdserverpb::ResponseOp) -> Self {
        use etcdserverpb::response_op::Response;
        match resp.response.take().unwrap() {
            Response::ResponseRange(r) => Self::Range(From::from(r)),
            Response::ResponsePut(r) => Self::Put(From::from(r)),
            Response::ResponseTxn(r) => Self::Txn(From::from(r)),
            Response::ResponseDeleteRange(r) => Self::Delete(From::from(r)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxnResponse {
    pub header: ResponseHeader,
    pub succeeded: bool,
    pub responses: Vec<TxnOpResponse>,
}

impl From<etcdserverpb::TxnResponse> for TxnResponse {
    fn from(proto: etcdserverpb::TxnResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            succeeded: proto.succeeded,
            responses: proto.responses.into_iter().map(From::from).collect(),
        }
    }
}
