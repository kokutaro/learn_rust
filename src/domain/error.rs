use crate::domain::tickets::ticket_error::TicketError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Ticket error: {0}")]
    Ticket(#[from] TicketError),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Concurrent modification error")]
    ConcurrentModification,
    #[error("Infrastructure error: {0}")]
    Infrastructure(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Invalid ticket id")]
    InvalidTicketId,
}

pub type Result<T> = std::result::Result<T, DomainError>;
