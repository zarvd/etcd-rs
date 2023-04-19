#[allow(warnings)]
pub mod mvccpb {
    tonic::include_proto!("mvccpb");
}

#[allow(warnings)]
pub mod authpb {
    tonic::include_proto!("authpb");
}

#[allow(warnings)]
pub mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

#[allow(warnings)]
pub mod v3lockpb {
    tonic::include_proto!("v3lockpb");
}

#[allow(warnings)]
pub mod v3electionpb {
    tonic::include_proto!("v3electionpb");
}
