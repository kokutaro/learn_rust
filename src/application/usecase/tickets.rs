use crate::domain::tickets::repository::{UowFactory, UowFactoryExt};
use crate::domain::tickets::ticket_error::TicketError;
use crate::{domain::error::Result, domain::tickets::ticket::Ticket};
use tracing::{instrument, Instrument};
use uuid::Uuid;

#[instrument(skip(fac), fields(ticket.id = tracing::field::Empty))]
pub async fn create_ticket(
    fac: &dyn UowFactory,
    title: String,
    description: String,
) -> Result<Uuid> {
    tracing::info!(title = %title,"Creating ticket");
    let ticket = Ticket::new(title, description, None)?;
    let ticket_id = ticket.id();
    fac.execute_in_transaction(async move |uow| {
        let mut repo = uow.ticket_repo();
        repo.insert(ticket.clone()).await?;
        Ok(())
    })
    .await?;
    tracing::Span::current().record("ticket.id", &ticket_id.value().to_string());
    tracing::info!(ticket.id = %ticket_id, "Ticket created");

    Ok(ticket_id.value())
}

#[instrument(skip(fac), fields(ticket.id = %id))]
pub async fn close_ticket(fac: &dyn UowFactory, id: Uuid) -> Result<()> {
    tracing::info!(id = %id, "Closing ticket");
    fac.execute_in_transaction(async move |uow| {
        let mut repo = uow.ticket_repo();
        let span = tracing::info_span!("close_ticket_task", %id);
        async {
            tracing::info!(ticket.id = %id, "Finding ticket by id");
            let mut ticket = repo
                .find_by_id(id.into())
                .await
                .map_err(|_| TicketError::NotFound)?;
            tracing::info!(ticket.id = %ticket.id(), "Closing ticket");
            ticket.close();
            repo.save(ticket.clone()).await?;
            Ok(())
        }
        .instrument(span)
        .await
    })
    .await?;

    tracing::Span::current().record("ticket.id", &id.to_string());
    tracing::info!(ticket.id = %id, "Ticket closed");
    Ok(())
}
