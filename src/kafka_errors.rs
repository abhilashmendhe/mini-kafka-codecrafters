
use std::net::SocketAddr;

use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Error)]
pub enum KafkaErrors {

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("SendError: {0}")]
    TokioChSendError(#[from] SendError<(SocketAddr, Vec<u8>)>),
}   