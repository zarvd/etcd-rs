use crate::proto::etcdserverpb;
use etcdserverpb::compare::{CompareResult, CompareTarget, TargetUnion};
use etcdserverpb::Compare;

pub struct TxnRequest {
    pub(crate) proto: etcdserverpb::TxnRequest,
}

impl TxnRequest {
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::TxnRequest {
                compare: vec![],
                success: vec![],
                failure: vec![],
            },
        }
    }

    pub fn when_version<K>(mut self, key: K, cmp: TxnCmp, version: usize) -> Self
    where
        K: Into<Vec<u8>>,
    {
        let result: CompareResult = cmp.into();
        self.proto.compare.push(Compare {
            result: result as i32,
            target: CompareTarget::Version as i32,
            key: key.into(),
            range_end: vec![],
            target_union: Some(TargetUnion::Version(version as i64)),
        });
        self
    }
}

pub enum TxnCmp {
    Equal,
    NotEqual,
    Greater,
    Less,
}

impl Into<CompareResult> for TxnCmp {
    fn into(self) -> CompareResult {
        match self {
            TxnCmp::Equal => CompareResult::Equal,
            TxnCmp::NotEqual => CompareResult::NotEqual,
            TxnCmp::Greater => CompareResult::Greater,
            TxnCmp::Less => CompareResult::Less,
        }
    }
}

pub struct TxnResponse {
    proto: etcdserverpb::TxnResponse,
}
