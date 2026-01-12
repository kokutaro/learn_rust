use crate::domain::tickets::ticket_error::TicketError;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TicketStatus {
    #[default]
    Open,
    Assigned { user_id: uuid::Uuid },
    Closed,
}

impl Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TicketStatus {
    type Err = TicketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(TicketStatus::Open),
            "closed" => Ok(TicketStatus::Closed),
            "assigned" => Ok(TicketStatus::Assigned { user_id: uuid::Uuid::new_v4() }),
            _ => Err(TicketError::InvalidStatus),
        }
    }
}