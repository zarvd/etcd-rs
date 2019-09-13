// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KV_RANGE: ::grpcio::Method<super::rpc::RangeRequest, super::rpc::RangeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.KV/Range",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_PUT: ::grpcio::Method<super::rpc::PutRequest, super::rpc::PutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.KV/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_DELETE_RANGE: ::grpcio::Method<super::rpc::DeleteRangeRequest, super::rpc::DeleteRangeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.KV/DeleteRange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_TXN: ::grpcio::Method<super::rpc::TxnRequest, super::rpc::TxnResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.KV/Txn",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_COMPACT: ::grpcio::Method<super::rpc::CompactionRequest, super::rpc::CompactionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.KV/Compact",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KvClient {
    client: ::grpcio::Client,
}

impl KvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn range_opt(&self, req: &super::rpc::RangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::RangeResponse> {
        self.client.unary_call(&METHOD_KV_RANGE, req, opt)
    }

    pub fn range(&self, req: &super::rpc::RangeRequest) -> ::grpcio::Result<super::rpc::RangeResponse> {
        self.range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn range_async_opt(&self, req: &super::rpc::RangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::RangeResponse>> {
        self.client.unary_call_async(&METHOD_KV_RANGE, req, opt)
    }

    pub fn range_async(&self, req: &super::rpc::RangeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::RangeResponse>> {
        self.range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::rpc::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::PutResponse> {
        self.client.unary_call(&METHOD_KV_PUT, req, opt)
    }

    pub fn put(&self, req: &super::rpc::PutRequest) -> ::grpcio::Result<super::rpc::PutResponse> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::rpc::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PutResponse>> {
        self.client.unary_call_async(&METHOD_KV_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::rpc::PutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PutResponse>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_range_opt(&self, req: &super::rpc::DeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DeleteRangeResponse> {
        self.client.unary_call(&METHOD_KV_DELETE_RANGE, req, opt)
    }

    pub fn delete_range(&self, req: &super::rpc::DeleteRangeRequest) -> ::grpcio::Result<super::rpc::DeleteRangeResponse> {
        self.delete_range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_range_async_opt(&self, req: &super::rpc::DeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteRangeResponse>> {
        self.client.unary_call_async(&METHOD_KV_DELETE_RANGE, req, opt)
    }

    pub fn delete_range_async(&self, req: &super::rpc::DeleteRangeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteRangeResponse>> {
        self.delete_range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn txn_opt(&self, req: &super::rpc::TxnRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::TxnResponse> {
        self.client.unary_call(&METHOD_KV_TXN, req, opt)
    }

    pub fn txn(&self, req: &super::rpc::TxnRequest) -> ::grpcio::Result<super::rpc::TxnResponse> {
        self.txn_opt(req, ::grpcio::CallOption::default())
    }

    pub fn txn_async_opt(&self, req: &super::rpc::TxnRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::TxnResponse>> {
        self.client.unary_call_async(&METHOD_KV_TXN, req, opt)
    }

    pub fn txn_async(&self, req: &super::rpc::TxnRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::TxnResponse>> {
        self.txn_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_opt(&self, req: &super::rpc::CompactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::CompactionResponse> {
        self.client.unary_call(&METHOD_KV_COMPACT, req, opt)
    }

    pub fn compact(&self, req: &super::rpc::CompactionRequest) -> ::grpcio::Result<super::rpc::CompactionResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_async_opt(&self, req: &super::rpc::CompactionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::CompactionResponse>> {
        self.client.unary_call_async(&METHOD_KV_COMPACT, req, opt)
    }

    pub fn compact_async(&self, req: &super::rpc::CompactionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::CompactionResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Kv {
    fn range(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::RangeRequest, sink: ::grpcio::UnarySink<super::rpc::RangeResponse>);
    fn put(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PutRequest, sink: ::grpcio::UnarySink<super::rpc::PutResponse>);
    fn delete_range(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DeleteRangeRequest, sink: ::grpcio::UnarySink<super::rpc::DeleteRangeResponse>);
    fn txn(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::TxnRequest, sink: ::grpcio::UnarySink<super::rpc::TxnResponse>);
    fn compact(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::CompactionRequest, sink: ::grpcio::UnarySink<super::rpc::CompactionResponse>);
}

pub fn create_kv<S: Kv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_RANGE, move |ctx, req, resp| {
        instance.range(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_DELETE_RANGE, move |ctx, req, resp| {
        instance.delete_range(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_TXN, move |ctx, req, resp| {
        instance.txn(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    builder.build()
}

const METHOD_WATCH_WATCH: ::grpcio::Method<super::rpc::WatchRequest, super::rpc::WatchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/etcdserverpb.Watch/Watch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct WatchClient {
    client: ::grpcio::Client,
}

impl WatchClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        WatchClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn watch_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::WatchRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::WatchResponse>)> {
        self.client.duplex_streaming(&METHOD_WATCH_WATCH, opt)
    }

    pub fn watch(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::WatchRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::WatchResponse>)> {
        self.watch_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Watch {
    fn watch(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::rpc::WatchRequest>, sink: ::grpcio::DuplexSink<super::rpc::WatchResponse>);
}

pub fn create_watch<S: Watch + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_WATCH_WATCH, move |ctx, req, resp| {
        instance.watch(ctx, req, resp)
    });
    builder.build()
}

const METHOD_LEASE_LEASE_GRANT: ::grpcio::Method<super::rpc::LeaseGrantRequest, super::rpc::LeaseGrantResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Lease/LeaseGrant",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LEASE_LEASE_REVOKE: ::grpcio::Method<super::rpc::LeaseRevokeRequest, super::rpc::LeaseRevokeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Lease/LeaseRevoke",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LEASE_LEASE_KEEP_ALIVE: ::grpcio::Method<super::rpc::LeaseKeepAliveRequest, super::rpc::LeaseKeepAliveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/etcdserverpb.Lease/LeaseKeepAlive",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LEASE_LEASE_TIME_TO_LIVE: ::grpcio::Method<super::rpc::LeaseTimeToLiveRequest, super::rpc::LeaseTimeToLiveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Lease/LeaseTimeToLive",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LeaseClient {
    client: ::grpcio::Client,
}

impl LeaseClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LeaseClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn lease_grant_opt(&self, req: &super::rpc::LeaseGrantRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::LeaseGrantResponse> {
        self.client.unary_call(&METHOD_LEASE_LEASE_GRANT, req, opt)
    }

    pub fn lease_grant(&self, req: &super::rpc::LeaseGrantRequest) -> ::grpcio::Result<super::rpc::LeaseGrantResponse> {
        self.lease_grant_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lease_grant_async_opt(&self, req: &super::rpc::LeaseGrantRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseGrantResponse>> {
        self.client.unary_call_async(&METHOD_LEASE_LEASE_GRANT, req, opt)
    }

    pub fn lease_grant_async(&self, req: &super::rpc::LeaseGrantRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseGrantResponse>> {
        self.lease_grant_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lease_revoke_opt(&self, req: &super::rpc::LeaseRevokeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::LeaseRevokeResponse> {
        self.client.unary_call(&METHOD_LEASE_LEASE_REVOKE, req, opt)
    }

    pub fn lease_revoke(&self, req: &super::rpc::LeaseRevokeRequest) -> ::grpcio::Result<super::rpc::LeaseRevokeResponse> {
        self.lease_revoke_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lease_revoke_async_opt(&self, req: &super::rpc::LeaseRevokeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseRevokeResponse>> {
        self.client.unary_call_async(&METHOD_LEASE_LEASE_REVOKE, req, opt)
    }

    pub fn lease_revoke_async(&self, req: &super::rpc::LeaseRevokeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseRevokeResponse>> {
        self.lease_revoke_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lease_keep_alive_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::LeaseKeepAliveRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::LeaseKeepAliveResponse>)> {
        self.client.duplex_streaming(&METHOD_LEASE_LEASE_KEEP_ALIVE, opt)
    }

    pub fn lease_keep_alive(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::LeaseKeepAliveRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::LeaseKeepAliveResponse>)> {
        self.lease_keep_alive_opt(::grpcio::CallOption::default())
    }

    pub fn lease_time_to_live_opt(&self, req: &super::rpc::LeaseTimeToLiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::LeaseTimeToLiveResponse> {
        self.client.unary_call(&METHOD_LEASE_LEASE_TIME_TO_LIVE, req, opt)
    }

    pub fn lease_time_to_live(&self, req: &super::rpc::LeaseTimeToLiveRequest) -> ::grpcio::Result<super::rpc::LeaseTimeToLiveResponse> {
        self.lease_time_to_live_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lease_time_to_live_async_opt(&self, req: &super::rpc::LeaseTimeToLiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseTimeToLiveResponse>> {
        self.client.unary_call_async(&METHOD_LEASE_LEASE_TIME_TO_LIVE, req, opt)
    }

    pub fn lease_time_to_live_async(&self, req: &super::rpc::LeaseTimeToLiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::LeaseTimeToLiveResponse>> {
        self.lease_time_to_live_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Lease {
    fn lease_grant(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::LeaseGrantRequest, sink: ::grpcio::UnarySink<super::rpc::LeaseGrantResponse>);
    fn lease_revoke(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::LeaseRevokeRequest, sink: ::grpcio::UnarySink<super::rpc::LeaseRevokeResponse>);
    fn lease_keep_alive(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::rpc::LeaseKeepAliveRequest>, sink: ::grpcio::DuplexSink<super::rpc::LeaseKeepAliveResponse>);
    fn lease_time_to_live(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::LeaseTimeToLiveRequest, sink: ::grpcio::UnarySink<super::rpc::LeaseTimeToLiveResponse>);
}

pub fn create_lease<S: Lease + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LEASE_LEASE_GRANT, move |ctx, req, resp| {
        instance.lease_grant(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LEASE_LEASE_REVOKE, move |ctx, req, resp| {
        instance.lease_revoke(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LEASE_LEASE_KEEP_ALIVE, move |ctx, req, resp| {
        instance.lease_keep_alive(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LEASE_LEASE_TIME_TO_LIVE, move |ctx, req, resp| {
        instance.lease_time_to_live(ctx, req, resp)
    });
    builder.build()
}

const METHOD_CLUSTER_MEMBER_ADD: ::grpcio::Method<super::rpc::MemberAddRequest, super::rpc::MemberAddResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Cluster/MemberAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MEMBER_REMOVE: ::grpcio::Method<super::rpc::MemberRemoveRequest, super::rpc::MemberRemoveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Cluster/MemberRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MEMBER_UPDATE: ::grpcio::Method<super::rpc::MemberUpdateRequest, super::rpc::MemberUpdateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Cluster/MemberUpdate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MEMBER_LIST: ::grpcio::Method<super::rpc::MemberListRequest, super::rpc::MemberListResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Cluster/MemberList",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ClusterClient {
    client: ::grpcio::Client,
}

impl ClusterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ClusterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn member_add_opt(&self, req: &super::rpc::MemberAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::MemberAddResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MEMBER_ADD, req, opt)
    }

    pub fn member_add(&self, req: &super::rpc::MemberAddRequest) -> ::grpcio::Result<super::rpc::MemberAddResponse> {
        self.member_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_add_async_opt(&self, req: &super::rpc::MemberAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberAddResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MEMBER_ADD, req, opt)
    }

    pub fn member_add_async(&self, req: &super::rpc::MemberAddRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberAddResponse>> {
        self.member_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_remove_opt(&self, req: &super::rpc::MemberRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::MemberRemoveResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MEMBER_REMOVE, req, opt)
    }

    pub fn member_remove(&self, req: &super::rpc::MemberRemoveRequest) -> ::grpcio::Result<super::rpc::MemberRemoveResponse> {
        self.member_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_remove_async_opt(&self, req: &super::rpc::MemberRemoveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberRemoveResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MEMBER_REMOVE, req, opt)
    }

    pub fn member_remove_async(&self, req: &super::rpc::MemberRemoveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberRemoveResponse>> {
        self.member_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_update_opt(&self, req: &super::rpc::MemberUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::MemberUpdateResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MEMBER_UPDATE, req, opt)
    }

    pub fn member_update(&self, req: &super::rpc::MemberUpdateRequest) -> ::grpcio::Result<super::rpc::MemberUpdateResponse> {
        self.member_update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_update_async_opt(&self, req: &super::rpc::MemberUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberUpdateResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MEMBER_UPDATE, req, opt)
    }

    pub fn member_update_async(&self, req: &super::rpc::MemberUpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberUpdateResponse>> {
        self.member_update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_list_opt(&self, req: &super::rpc::MemberListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::MemberListResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MEMBER_LIST, req, opt)
    }

    pub fn member_list(&self, req: &super::rpc::MemberListRequest) -> ::grpcio::Result<super::rpc::MemberListResponse> {
        self.member_list_opt(req, ::grpcio::CallOption::default())
    }

    pub fn member_list_async_opt(&self, req: &super::rpc::MemberListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberListResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MEMBER_LIST, req, opt)
    }

    pub fn member_list_async(&self, req: &super::rpc::MemberListRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MemberListResponse>> {
        self.member_list_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Cluster {
    fn member_add(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::MemberAddRequest, sink: ::grpcio::UnarySink<super::rpc::MemberAddResponse>);
    fn member_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::MemberRemoveRequest, sink: ::grpcio::UnarySink<super::rpc::MemberRemoveResponse>);
    fn member_update(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::MemberUpdateRequest, sink: ::grpcio::UnarySink<super::rpc::MemberUpdateResponse>);
    fn member_list(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::MemberListRequest, sink: ::grpcio::UnarySink<super::rpc::MemberListResponse>);
}

pub fn create_cluster<S: Cluster + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MEMBER_ADD, move |ctx, req, resp| {
        instance.member_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MEMBER_REMOVE, move |ctx, req, resp| {
        instance.member_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MEMBER_UPDATE, move |ctx, req, resp| {
        instance.member_update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MEMBER_LIST, move |ctx, req, resp| {
        instance.member_list(ctx, req, resp)
    });
    builder.build()
}

const METHOD_MAINTENANCE_ALARM: ::grpcio::Method<super::rpc::AlarmRequest, super::rpc::AlarmResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/Alarm",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_STATUS: ::grpcio::Method<super::rpc::StatusRequest, super::rpc::StatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/Status",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_DEFRAGMENT: ::grpcio::Method<super::rpc::DefragmentRequest, super::rpc::DefragmentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/Defragment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_HASH: ::grpcio::Method<super::rpc::HashRequest, super::rpc::HashResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/Hash",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_HASH_KV: ::grpcio::Method<super::rpc::HashKVRequest, super::rpc::HashKVResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/HashKV",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_SNAPSHOT: ::grpcio::Method<super::rpc::SnapshotRequest, super::rpc::SnapshotResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/etcdserverpb.Maintenance/Snapshot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MAINTENANCE_MOVE_LEADER: ::grpcio::Method<super::rpc::MoveLeaderRequest, super::rpc::MoveLeaderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Maintenance/MoveLeader",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MaintenanceClient {
    client: ::grpcio::Client,
}

impl MaintenanceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MaintenanceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn alarm_opt(&self, req: &super::rpc::AlarmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AlarmResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_ALARM, req, opt)
    }

    pub fn alarm(&self, req: &super::rpc::AlarmRequest) -> ::grpcio::Result<super::rpc::AlarmResponse> {
        self.alarm_opt(req, ::grpcio::CallOption::default())
    }

    pub fn alarm_async_opt(&self, req: &super::rpc::AlarmRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AlarmResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_ALARM, req, opt)
    }

    pub fn alarm_async(&self, req: &super::rpc::AlarmRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AlarmResponse>> {
        self.alarm_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn status_opt(&self, req: &super::rpc::StatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::StatusResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_STATUS, req, opt)
    }

    pub fn status(&self, req: &super::rpc::StatusRequest) -> ::grpcio::Result<super::rpc::StatusResponse> {
        self.status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn status_async_opt(&self, req: &super::rpc::StatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::StatusResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_STATUS, req, opt)
    }

    pub fn status_async(&self, req: &super::rpc::StatusRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::StatusResponse>> {
        self.status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn defragment_opt(&self, req: &super::rpc::DefragmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DefragmentResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_DEFRAGMENT, req, opt)
    }

    pub fn defragment(&self, req: &super::rpc::DefragmentRequest) -> ::grpcio::Result<super::rpc::DefragmentResponse> {
        self.defragment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn defragment_async_opt(&self, req: &super::rpc::DefragmentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DefragmentResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_DEFRAGMENT, req, opt)
    }

    pub fn defragment_async(&self, req: &super::rpc::DefragmentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DefragmentResponse>> {
        self.defragment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn hash_opt(&self, req: &super::rpc::HashRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::HashResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_HASH, req, opt)
    }

    pub fn hash(&self, req: &super::rpc::HashRequest) -> ::grpcio::Result<super::rpc::HashResponse> {
        self.hash_opt(req, ::grpcio::CallOption::default())
    }

    pub fn hash_async_opt(&self, req: &super::rpc::HashRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HashResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_HASH, req, opt)
    }

    pub fn hash_async(&self, req: &super::rpc::HashRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HashResponse>> {
        self.hash_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn hash_kv_opt(&self, req: &super::rpc::HashKVRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::HashKVResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_HASH_KV, req, opt)
    }

    pub fn hash_kv(&self, req: &super::rpc::HashKVRequest) -> ::grpcio::Result<super::rpc::HashKVResponse> {
        self.hash_kv_opt(req, ::grpcio::CallOption::default())
    }

    pub fn hash_kv_async_opt(&self, req: &super::rpc::HashKVRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HashKVResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_HASH_KV, req, opt)
    }

    pub fn hash_kv_async(&self, req: &super::rpc::HashKVRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HashKVResponse>> {
        self.hash_kv_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn snapshot_opt(&self, req: &super::rpc::SnapshotRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::SnapshotResponse>> {
        self.client.server_streaming(&METHOD_MAINTENANCE_SNAPSHOT, req, opt)
    }

    pub fn snapshot(&self, req: &super::rpc::SnapshotRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::SnapshotResponse>> {
        self.snapshot_opt(req, ::grpcio::CallOption::default())
    }

    pub fn move_leader_opt(&self, req: &super::rpc::MoveLeaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::MoveLeaderResponse> {
        self.client.unary_call(&METHOD_MAINTENANCE_MOVE_LEADER, req, opt)
    }

    pub fn move_leader(&self, req: &super::rpc::MoveLeaderRequest) -> ::grpcio::Result<super::rpc::MoveLeaderResponse> {
        self.move_leader_opt(req, ::grpcio::CallOption::default())
    }

    pub fn move_leader_async_opt(&self, req: &super::rpc::MoveLeaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MoveLeaderResponse>> {
        self.client.unary_call_async(&METHOD_MAINTENANCE_MOVE_LEADER, req, opt)
    }

    pub fn move_leader_async(&self, req: &super::rpc::MoveLeaderRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::MoveLeaderResponse>> {
        self.move_leader_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Maintenance {
    fn alarm(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AlarmRequest, sink: ::grpcio::UnarySink<super::rpc::AlarmResponse>);
    fn status(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::StatusRequest, sink: ::grpcio::UnarySink<super::rpc::StatusResponse>);
    fn defragment(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DefragmentRequest, sink: ::grpcio::UnarySink<super::rpc::DefragmentResponse>);
    fn hash(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::HashRequest, sink: ::grpcio::UnarySink<super::rpc::HashResponse>);
    fn hash_kv(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::HashKVRequest, sink: ::grpcio::UnarySink<super::rpc::HashKVResponse>);
    fn snapshot(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SnapshotRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::SnapshotResponse>);
    fn move_leader(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::MoveLeaderRequest, sink: ::grpcio::UnarySink<super::rpc::MoveLeaderResponse>);
}

pub fn create_maintenance<S: Maintenance + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_ALARM, move |ctx, req, resp| {
        instance.alarm(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_STATUS, move |ctx, req, resp| {
        instance.status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_DEFRAGMENT, move |ctx, req, resp| {
        instance.defragment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_HASH, move |ctx, req, resp| {
        instance.hash(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_HASH_KV, move |ctx, req, resp| {
        instance.hash_kv(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_MAINTENANCE_SNAPSHOT, move |ctx, req, resp| {
        instance.snapshot(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MAINTENANCE_MOVE_LEADER, move |ctx, req, resp| {
        instance.move_leader(ctx, req, resp)
    });
    builder.build()
}

const METHOD_AUTH_AUTH_ENABLE: ::grpcio::Method<super::rpc::AuthEnableRequest, super::rpc::AuthEnableResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/AuthEnable",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_AUTH_DISABLE: ::grpcio::Method<super::rpc::AuthDisableRequest, super::rpc::AuthDisableResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/AuthDisable",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_AUTHENTICATE: ::grpcio::Method<super::rpc::AuthenticateRequest, super::rpc::AuthenticateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/Authenticate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_ADD: ::grpcio::Method<super::rpc::AuthUserAddRequest, super::rpc::AuthUserAddResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_GET: ::grpcio::Method<super::rpc::AuthUserGetRequest, super::rpc::AuthUserGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_LIST: ::grpcio::Method<super::rpc::AuthUserListRequest, super::rpc::AuthUserListResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserList",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_DELETE: ::grpcio::Method<super::rpc::AuthUserDeleteRequest, super::rpc::AuthUserDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_CHANGE_PASSWORD: ::grpcio::Method<super::rpc::AuthUserChangePasswordRequest, super::rpc::AuthUserChangePasswordResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserChangePassword",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_GRANT_ROLE: ::grpcio::Method<super::rpc::AuthUserGrantRoleRequest, super::rpc::AuthUserGrantRoleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserGrantRole",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_USER_REVOKE_ROLE: ::grpcio::Method<super::rpc::AuthUserRevokeRoleRequest, super::rpc::AuthUserRevokeRoleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/UserRevokeRole",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_ADD: ::grpcio::Method<super::rpc::AuthRoleAddRequest, super::rpc::AuthRoleAddResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_GET: ::grpcio::Method<super::rpc::AuthRoleGetRequest, super::rpc::AuthRoleGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_LIST: ::grpcio::Method<super::rpc::AuthRoleListRequest, super::rpc::AuthRoleListResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleList",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_DELETE: ::grpcio::Method<super::rpc::AuthRoleDeleteRequest, super::rpc::AuthRoleDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_GRANT_PERMISSION: ::grpcio::Method<super::rpc::AuthRoleGrantPermissionRequest, super::rpc::AuthRoleGrantPermissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleGrantPermission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTH_ROLE_REVOKE_PERMISSION: ::grpcio::Method<super::rpc::AuthRoleRevokePermissionRequest, super::rpc::AuthRoleRevokePermissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/etcdserverpb.Auth/RoleRevokePermission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AuthClient {
    client: ::grpcio::Client,
}

impl AuthClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AuthClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn auth_enable_opt(&self, req: &super::rpc::AuthEnableRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthEnableResponse> {
        self.client.unary_call(&METHOD_AUTH_AUTH_ENABLE, req, opt)
    }

    pub fn auth_enable(&self, req: &super::rpc::AuthEnableRequest) -> ::grpcio::Result<super::rpc::AuthEnableResponse> {
        self.auth_enable_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_enable_async_opt(&self, req: &super::rpc::AuthEnableRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthEnableResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_AUTH_ENABLE, req, opt)
    }

    pub fn auth_enable_async(&self, req: &super::rpc::AuthEnableRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthEnableResponse>> {
        self.auth_enable_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_disable_opt(&self, req: &super::rpc::AuthDisableRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthDisableResponse> {
        self.client.unary_call(&METHOD_AUTH_AUTH_DISABLE, req, opt)
    }

    pub fn auth_disable(&self, req: &super::rpc::AuthDisableRequest) -> ::grpcio::Result<super::rpc::AuthDisableResponse> {
        self.auth_disable_opt(req, ::grpcio::CallOption::default())
    }

    pub fn auth_disable_async_opt(&self, req: &super::rpc::AuthDisableRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthDisableResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_AUTH_DISABLE, req, opt)
    }

    pub fn auth_disable_async(&self, req: &super::rpc::AuthDisableRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthDisableResponse>> {
        self.auth_disable_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authenticate_opt(&self, req: &super::rpc::AuthenticateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthenticateResponse> {
        self.client.unary_call(&METHOD_AUTH_AUTHENTICATE, req, opt)
    }

    pub fn authenticate(&self, req: &super::rpc::AuthenticateRequest) -> ::grpcio::Result<super::rpc::AuthenticateResponse> {
        self.authenticate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authenticate_async_opt(&self, req: &super::rpc::AuthenticateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthenticateResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_AUTHENTICATE, req, opt)
    }

    pub fn authenticate_async(&self, req: &super::rpc::AuthenticateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthenticateResponse>> {
        self.authenticate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_add_opt(&self, req: &super::rpc::AuthUserAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserAddResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_ADD, req, opt)
    }

    pub fn user_add(&self, req: &super::rpc::AuthUserAddRequest) -> ::grpcio::Result<super::rpc::AuthUserAddResponse> {
        self.user_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_add_async_opt(&self, req: &super::rpc::AuthUserAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserAddResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_ADD, req, opt)
    }

    pub fn user_add_async(&self, req: &super::rpc::AuthUserAddRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserAddResponse>> {
        self.user_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_get_opt(&self, req: &super::rpc::AuthUserGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserGetResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_GET, req, opt)
    }

    pub fn user_get(&self, req: &super::rpc::AuthUserGetRequest) -> ::grpcio::Result<super::rpc::AuthUserGetResponse> {
        self.user_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_get_async_opt(&self, req: &super::rpc::AuthUserGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserGetResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_GET, req, opt)
    }

    pub fn user_get_async(&self, req: &super::rpc::AuthUserGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserGetResponse>> {
        self.user_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_list_opt(&self, req: &super::rpc::AuthUserListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserListResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_LIST, req, opt)
    }

    pub fn user_list(&self, req: &super::rpc::AuthUserListRequest) -> ::grpcio::Result<super::rpc::AuthUserListResponse> {
        self.user_list_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_list_async_opt(&self, req: &super::rpc::AuthUserListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserListResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_LIST, req, opt)
    }

    pub fn user_list_async(&self, req: &super::rpc::AuthUserListRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserListResponse>> {
        self.user_list_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_delete_opt(&self, req: &super::rpc::AuthUserDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserDeleteResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_DELETE, req, opt)
    }

    pub fn user_delete(&self, req: &super::rpc::AuthUserDeleteRequest) -> ::grpcio::Result<super::rpc::AuthUserDeleteResponse> {
        self.user_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_delete_async_opt(&self, req: &super::rpc::AuthUserDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserDeleteResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_DELETE, req, opt)
    }

    pub fn user_delete_async(&self, req: &super::rpc::AuthUserDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserDeleteResponse>> {
        self.user_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_change_password_opt(&self, req: &super::rpc::AuthUserChangePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserChangePasswordResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_CHANGE_PASSWORD, req, opt)
    }

    pub fn user_change_password(&self, req: &super::rpc::AuthUserChangePasswordRequest) -> ::grpcio::Result<super::rpc::AuthUserChangePasswordResponse> {
        self.user_change_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_change_password_async_opt(&self, req: &super::rpc::AuthUserChangePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserChangePasswordResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_CHANGE_PASSWORD, req, opt)
    }

    pub fn user_change_password_async(&self, req: &super::rpc::AuthUserChangePasswordRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserChangePasswordResponse>> {
        self.user_change_password_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_grant_role_opt(&self, req: &super::rpc::AuthUserGrantRoleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserGrantRoleResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_GRANT_ROLE, req, opt)
    }

    pub fn user_grant_role(&self, req: &super::rpc::AuthUserGrantRoleRequest) -> ::grpcio::Result<super::rpc::AuthUserGrantRoleResponse> {
        self.user_grant_role_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_grant_role_async_opt(&self, req: &super::rpc::AuthUserGrantRoleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserGrantRoleResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_GRANT_ROLE, req, opt)
    }

    pub fn user_grant_role_async(&self, req: &super::rpc::AuthUserGrantRoleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserGrantRoleResponse>> {
        self.user_grant_role_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_revoke_role_opt(&self, req: &super::rpc::AuthUserRevokeRoleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthUserRevokeRoleResponse> {
        self.client.unary_call(&METHOD_AUTH_USER_REVOKE_ROLE, req, opt)
    }

    pub fn user_revoke_role(&self, req: &super::rpc::AuthUserRevokeRoleRequest) -> ::grpcio::Result<super::rpc::AuthUserRevokeRoleResponse> {
        self.user_revoke_role_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_revoke_role_async_opt(&self, req: &super::rpc::AuthUserRevokeRoleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserRevokeRoleResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_USER_REVOKE_ROLE, req, opt)
    }

    pub fn user_revoke_role_async(&self, req: &super::rpc::AuthUserRevokeRoleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthUserRevokeRoleResponse>> {
        self.user_revoke_role_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_add_opt(&self, req: &super::rpc::AuthRoleAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleAddResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_ADD, req, opt)
    }

    pub fn role_add(&self, req: &super::rpc::AuthRoleAddRequest) -> ::grpcio::Result<super::rpc::AuthRoleAddResponse> {
        self.role_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_add_async_opt(&self, req: &super::rpc::AuthRoleAddRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleAddResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_ADD, req, opt)
    }

    pub fn role_add_async(&self, req: &super::rpc::AuthRoleAddRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleAddResponse>> {
        self.role_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_get_opt(&self, req: &super::rpc::AuthRoleGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleGetResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_GET, req, opt)
    }

    pub fn role_get(&self, req: &super::rpc::AuthRoleGetRequest) -> ::grpcio::Result<super::rpc::AuthRoleGetResponse> {
        self.role_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_get_async_opt(&self, req: &super::rpc::AuthRoleGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleGetResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_GET, req, opt)
    }

    pub fn role_get_async(&self, req: &super::rpc::AuthRoleGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleGetResponse>> {
        self.role_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_list_opt(&self, req: &super::rpc::AuthRoleListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleListResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_LIST, req, opt)
    }

    pub fn role_list(&self, req: &super::rpc::AuthRoleListRequest) -> ::grpcio::Result<super::rpc::AuthRoleListResponse> {
        self.role_list_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_list_async_opt(&self, req: &super::rpc::AuthRoleListRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleListResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_LIST, req, opt)
    }

    pub fn role_list_async(&self, req: &super::rpc::AuthRoleListRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleListResponse>> {
        self.role_list_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_delete_opt(&self, req: &super::rpc::AuthRoleDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleDeleteResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_DELETE, req, opt)
    }

    pub fn role_delete(&self, req: &super::rpc::AuthRoleDeleteRequest) -> ::grpcio::Result<super::rpc::AuthRoleDeleteResponse> {
        self.role_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_delete_async_opt(&self, req: &super::rpc::AuthRoleDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleDeleteResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_DELETE, req, opt)
    }

    pub fn role_delete_async(&self, req: &super::rpc::AuthRoleDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleDeleteResponse>> {
        self.role_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_grant_permission_opt(&self, req: &super::rpc::AuthRoleGrantPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleGrantPermissionResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_GRANT_PERMISSION, req, opt)
    }

    pub fn role_grant_permission(&self, req: &super::rpc::AuthRoleGrantPermissionRequest) -> ::grpcio::Result<super::rpc::AuthRoleGrantPermissionResponse> {
        self.role_grant_permission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_grant_permission_async_opt(&self, req: &super::rpc::AuthRoleGrantPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleGrantPermissionResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_GRANT_PERMISSION, req, opt)
    }

    pub fn role_grant_permission_async(&self, req: &super::rpc::AuthRoleGrantPermissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleGrantPermissionResponse>> {
        self.role_grant_permission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_revoke_permission_opt(&self, req: &super::rpc::AuthRoleRevokePermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AuthRoleRevokePermissionResponse> {
        self.client.unary_call(&METHOD_AUTH_ROLE_REVOKE_PERMISSION, req, opt)
    }

    pub fn role_revoke_permission(&self, req: &super::rpc::AuthRoleRevokePermissionRequest) -> ::grpcio::Result<super::rpc::AuthRoleRevokePermissionResponse> {
        self.role_revoke_permission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn role_revoke_permission_async_opt(&self, req: &super::rpc::AuthRoleRevokePermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleRevokePermissionResponse>> {
        self.client.unary_call_async(&METHOD_AUTH_ROLE_REVOKE_PERMISSION, req, opt)
    }

    pub fn role_revoke_permission_async(&self, req: &super::rpc::AuthRoleRevokePermissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AuthRoleRevokePermissionResponse>> {
        self.role_revoke_permission_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Auth {
    fn auth_enable(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthEnableRequest, sink: ::grpcio::UnarySink<super::rpc::AuthEnableResponse>);
    fn auth_disable(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthDisableRequest, sink: ::grpcio::UnarySink<super::rpc::AuthDisableResponse>);
    fn authenticate(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthenticateRequest, sink: ::grpcio::UnarySink<super::rpc::AuthenticateResponse>);
    fn user_add(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserAddRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserAddResponse>);
    fn user_get(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserGetRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserGetResponse>);
    fn user_list(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserListRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserListResponse>);
    fn user_delete(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserDeleteRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserDeleteResponse>);
    fn user_change_password(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserChangePasswordRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserChangePasswordResponse>);
    fn user_grant_role(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserGrantRoleRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserGrantRoleResponse>);
    fn user_revoke_role(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthUserRevokeRoleRequest, sink: ::grpcio::UnarySink<super::rpc::AuthUserRevokeRoleResponse>);
    fn role_add(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleAddRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleAddResponse>);
    fn role_get(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleGetRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleGetResponse>);
    fn role_list(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleListRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleListResponse>);
    fn role_delete(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleDeleteRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleDeleteResponse>);
    fn role_grant_permission(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleGrantPermissionRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleGrantPermissionResponse>);
    fn role_revoke_permission(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AuthRoleRevokePermissionRequest, sink: ::grpcio::UnarySink<super::rpc::AuthRoleRevokePermissionResponse>);
}

pub fn create_auth<S: Auth + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_AUTH_ENABLE, move |ctx, req, resp| {
        instance.auth_enable(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_AUTH_DISABLE, move |ctx, req, resp| {
        instance.auth_disable(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_AUTHENTICATE, move |ctx, req, resp| {
        instance.authenticate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_ADD, move |ctx, req, resp| {
        instance.user_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_GET, move |ctx, req, resp| {
        instance.user_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_LIST, move |ctx, req, resp| {
        instance.user_list(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_DELETE, move |ctx, req, resp| {
        instance.user_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_CHANGE_PASSWORD, move |ctx, req, resp| {
        instance.user_change_password(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_GRANT_ROLE, move |ctx, req, resp| {
        instance.user_grant_role(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_USER_REVOKE_ROLE, move |ctx, req, resp| {
        instance.user_revoke_role(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_ADD, move |ctx, req, resp| {
        instance.role_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_GET, move |ctx, req, resp| {
        instance.role_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_LIST, move |ctx, req, resp| {
        instance.role_list(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_DELETE, move |ctx, req, resp| {
        instance.role_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_GRANT_PERMISSION, move |ctx, req, resp| {
        instance.role_grant_permission(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTH_ROLE_REVOKE_PERMISSION, move |ctx, req, resp| {
        instance.role_revoke_permission(ctx, req, resp)
    });
    builder.build()
}
