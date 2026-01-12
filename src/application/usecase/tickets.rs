use crate::domain::tickets::repository::{UowFactory, UowFactoryExt};
use crate::domain::tickets::ticket_error::TicketError;
use crate::{domain::error::Result, domain::tickets::ticket::Ticket};
use tracing::{info, instrument};
use uuid::Uuid;

#[instrument(skip(fac), fields(title = %title, description = %description))]
pub async fn create_ticket(
    fac: &dyn UowFactory,
    title: String,
    description: String,
) -> Result<Uuid> {
    let ticket = Ticket::new(title, description, None)?;
    let ticket_id = ticket.id();
    fac.execute_in_transaction(async move |uow| {
        info!("Creating ticket");
        let mut repo = uow.ticket_repo();
        repo.insert(ticket.clone()).await?;
        Ok(())
    })
    .await?;
    info!("Ticket created");
    Ok(ticket_id.value())
}

#[instrument(skip(fac), fields(id = %id))]
pub async fn close_ticket(fac: &dyn UowFactory, id: Uuid) -> Result<()> {
    fac.execute_in_transaction(async move |uow| {
        let mut repo = uow.ticket_repo();
        info!("Finding ticket by id:");
        let mut ticket = repo
            .find_by_id(id.into())
            .await
            .ok_or(TicketError::NotFound)?;
        info!("Closing ticket:");
        ticket.close();
        repo.save(ticket.clone()).await?;
        info!("Ticket closed");
        Ok(())
    })
    .await?;
    Ok(())
}
