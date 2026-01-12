use crate::domain::tickets::ticket_error::TicketError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicketDescription(String);

impl TicketDescription {
    fn new(description: String) -> Self {
        Self(description)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TicketDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketError;
    fn try_from(value: &str) -> Result<Self, TicketError> {
        validate(value)?;
        Ok(Self::new(value.to_string()))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketError;
    fn try_from(value: String) -> Result<Self, TicketError> {
        validate(&value)?;
        Ok(Self::new(value))
    }
}

fn validate(description: &str) -> Result<(), TicketError> {
    if description.trim().is_empty() {
        return Err(TicketError::EmptyDescription);
    }
    if description.len() > 100 {
        return Err(TicketError::TooLongDescription);
    }
    Ok(())
}
