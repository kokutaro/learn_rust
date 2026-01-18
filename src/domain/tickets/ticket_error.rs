use crate::domain::tickets::ticket_description::TicketDescriptionError;
use crate::domain::tickets::ticket_title::TicketTitleError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TicketError {
    #[error("Ticket title cannot be empty")]
    EmptyTitle,
    #[error("Ticket title cannot be longer than 100 characters")]
    TooLongTitle,
    #[error("Ticket description cannot be empty")]
    EmptyDescription,
    #[error("Ticket description cannot be longer than 500 characters")]
    TooLongDescription,
    #[error("Ticket not found")]
    NotFound,
    #[error("Invalid usecase status")]
    InvalidStatus,
    #[error("Ticket description error: {0}")]
    TicketDescriptionError(#[from] TicketDescriptionError),
    #[error("Ticket title error: {0}")]
    TicketTitleError(#[from] TicketTitleError),
}
