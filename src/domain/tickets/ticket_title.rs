use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 100),
    derive(Debug, Clone, PartialEq, Eq, AsRef, TryFrom)
)]
pub struct TicketTitle(String);