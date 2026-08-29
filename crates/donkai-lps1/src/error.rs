use thiserror::Error;

#[derive(Error, Debug)]
pub enum Lps1Error {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Hex encoding/decoding error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Invalid schema: expected {expected}, found {found}")]
    InvalidSchema { expected: String, found: String },

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Proof verification failed: {0}")]
    ProofVerification(String),

    #[error("Invalid leaf index {index} for tree of size {size}")]
    InvalidLeafIndex { index: usize, size: usize },

    #[error("IO error: {0}")]
    Io(String),

    #[error("Privacy policy violation: {0}")]
    PrivacyViolation(String),
}

pub type Result<T> = std::result::Result<T, Lps1Error>;
