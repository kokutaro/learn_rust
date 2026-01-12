use crate::domain::tickets::ticket_error::TicketError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicketTitle(String);

impl TicketTitle {
    fn new(title: String) -> Self {
        Self(title)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for TicketTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketError;
    fn try_from(value: &str) -> Result<Self, TicketError> {
        validate(value)?;
        Ok(Self::new(value.to_string()))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketError;
    fn try_from(value: String) -> Result<Self, TicketError> {
        validate(&value)?;
        Ok(Self::new(value))
    }
}

fn validate(title: &str) -> Result<(), TicketError> {
    if title.trim().is_empty() {
        return Err(TicketError::EmptyTitle);
    }
    if title.len() > 100 {
        return Err(TicketError::TooLongTitle);
    }
    Ok(())
}
