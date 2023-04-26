use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Application already got initialized!")]
    AlreadyInitialized
}