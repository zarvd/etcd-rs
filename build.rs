fn main() -> Result<(), Box<dyn std::error::Error>> {
    // NOTE: compile v3lockpb before rpc for disabling generate duplicated client in v3lockpb
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .compile(&["proto/lock.proto"], &["proto"])?;

    tonic_build::configure().build_server(false).compile(
        &["proto/auth.proto", "proto/kv.proto", "proto/rpc.proto"],
        &["proto"],
    )?;
    Ok(())
}
