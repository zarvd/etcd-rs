#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid URI")]
    InvalidURI(#[from] http::uri::InvalidUri),
    #[error("gRPC transport error")]
    Transport(#[from] tonic::transport::Error),
    #[error("response failed")]
    Response(#[from] tonic::Status),
    #[error("channel closed")]
    ChannelClosed,
}
