fn main() {
    println!("cargo:rerun-if-changed={}", "proto");
    protoc_grpcio::compile_grpc_protos(
        &[
            "auth.proto",
            "kv.proto",
            "rpc.proto",
            "election.proto",
            "lock.proto",
        ],
        &["proto"],
        "src/proto",
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
