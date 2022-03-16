use crate::proto::etcdserverpb;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("invalid URI")]
    InvalidURI(#[from] http::uri::InvalidUri),
    #[error("gRPC transport error")]
    Transport(#[from] tonic::transport::Error),
    #[error("response failed")]
    Response(#[from] tonic::Status),
    #[error("channel closed")]
    ChannelClosed,
    #[error("failed to create watch")]
    CreateWatch,
    #[error("unexpected watch event")]
    WatchEvent(String),
    #[error("failed to keep alive lease")]
    KeepAliveLease,
    #[error("watch channel send error")]
    WatchChannelSend(#[from] tokio::sync::mpsc::error::SendError<etcdserverpb::WatchRequest>),
    #[error("watch event exhausted")]
    WatchEventExhausted,
}
