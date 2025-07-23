
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KafkaErrors {

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}   