fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "proto/auth.proto",
            "proto/kv.proto",
            "proto/rpc.proto",
            "proto/v3lock.proto",
            "proto/v3election.proto",
        ],
        &["proto"],
    )?;

    Ok(())
}
