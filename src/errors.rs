use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollaboriError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Operation not found")]
    OperationNotFound,

    #[error("Conflict detected")]
    ConflictDetected,
}
