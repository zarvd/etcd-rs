/// Errors
#[derive(Debug)]
pub enum Error {
    GrpcFailure(grpcio::Error),
    Unknown,
}
