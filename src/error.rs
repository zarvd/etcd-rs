/// Errors
#[derive(Debug)]
pub enum Error {
    GrpcFailure(grpcio::Error),
    Unknown,
}

impl From<grpcio::Error> for Error {
    fn from(err: grpcio::Error) -> Self {
        Error::GrpcFailure(err)
    }
}