pub(crate) type AmpResult<T> = Result<T, AmpError>;

#[derive(Debug, thiserror::Error)]
pub enum AmpError {
    #[error("Bluetooth error: {0}")]
    Bluetooth(#[from] btleplug::Error),
    #[error("Thread failed execution: {0}")]
    Thread(#[from] tokio::task::JoinError),
}
