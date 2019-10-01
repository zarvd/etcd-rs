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

const METHOD_LOCK_LOCK: ::grpcio::Method<super::lock::LockRequest, super::lock::LockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/v3lockpb.Lock/Lock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOCK_UNLOCK: ::grpcio::Method<super::lock::UnlockRequest, super::lock::UnlockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/v3lockpb.Lock/Unlock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LockClient {
    client: ::grpcio::Client,
}

impl LockClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LockClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn lock_opt(&self, req: &super::lock::LockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lock::LockResponse> {
        self.client.unary_call(&METHOD_LOCK_LOCK, req, opt)
    }

    pub fn lock(&self, req: &super::lock::LockRequest) -> ::grpcio::Result<super::lock::LockResponse> {
        self.lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lock_async_opt(&self, req: &super::lock::LockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lock::LockResponse>> {
        self.client.unary_call_async(&METHOD_LOCK_LOCK, req, opt)
    }

    pub fn lock_async(&self, req: &super::lock::LockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lock::LockResponse>> {
        self.lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_opt(&self, req: &super::lock::UnlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lock::UnlockResponse> {
        self.client.unary_call(&METHOD_LOCK_UNLOCK, req, opt)
    }

    pub fn unlock(&self, req: &super::lock::UnlockRequest) -> ::grpcio::Result<super::lock::UnlockResponse> {
        self.unlock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_async_opt(&self, req: &super::lock::UnlockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lock::UnlockResponse>> {
        self.client.unary_call_async(&METHOD_LOCK_UNLOCK, req, opt)
    }

    pub fn unlock_async(&self, req: &super::lock::UnlockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lock::UnlockResponse>> {
        self.unlock_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Lock {
    fn lock(&mut self, ctx: ::grpcio::RpcContext, req: super::lock::LockRequest, sink: ::grpcio::UnarySink<super::lock::LockResponse>);
    fn unlock(&mut self, ctx: ::grpcio::RpcContext, req: super::lock::UnlockRequest, sink: ::grpcio::UnarySink<super::lock::UnlockResponse>);
}

pub fn create_lock<S: Lock + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOCK_LOCK, move |ctx, req, resp| {
        instance.lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOCK_UNLOCK, move |ctx, req, resp| {
        instance.unlock(ctx, req, resp)
    });
    builder.build()
}
