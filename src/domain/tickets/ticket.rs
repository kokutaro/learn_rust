use crate::domain::tickets::ticket_description::TicketDescription;
use crate::domain::tickets::ticket_error::TicketError;
pub(crate) use crate::domain::tickets::ticket_id::TicketId;
use crate::domain::tickets::ticket_status::TicketStatus;
use crate::domain::tickets::ticket_title::TicketTitle;

#[derive(Debug, Clone)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: TicketStatus,
    assignee: Option<uuid::Uuid>,
    version: i64,
}

impl Ticket {
    pub(crate) fn reconstruct(
        id: TicketId,
        title: impl TryInto<TicketTitle, Error=TicketError>,
        description: impl TryInto<TicketDescription, Error=TicketError>,
        status: TicketStatus,
        assignee: Option<uuid::Uuid>,
        version: i64,
    ) -> Self {
        Self {
            id,
            title: title.try_into().unwrap(),
            description: description.try_into().unwrap(),
            assignee,
            status,
            version,
        }
    }
}

impl Ticket {
    pub fn new(
        title: impl TryInto<TicketTitle, Error=TicketError>,
        description: impl TryInto<TicketDescription, Error=TicketError>,
        assignee: Option<uuid::Uuid>,
    ) -> Result<Self, TicketError> {
        Ok(Self {
            id: TicketId::new(),
            title: title.try_into()?,
            description: description.try_into()?,
            status: Default::default(),
            assignee,
            version: 0,
        })
    }

    pub fn assign(&mut self, user_id: uuid::Uuid) {
        self.status = TicketStatus::Assigned { user_id };
    }

    pub fn assignee(&self) -> Option<uuid::Uuid> {
        self.assignee
    }

    pub fn close(&mut self) {
        self.status = TicketStatus::Closed;
    }

    pub fn id(&self) -> TicketId {
        self.id
    }

    pub fn status(&self) -> TicketStatus {
        self.status
    }

    pub fn title(&self) -> String {
        self.title.to_string()
    }

    pub fn description(&self) -> String {
        self.description.to_string()
    }
    pub fn version(&self) -> i64 {
        self.version
    }
}
