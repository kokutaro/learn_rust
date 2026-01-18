use crate::domain::error::{DomainError, Result};
use crate::domain::tickets::ticket::Ticket;
use crate::domain::tickets::ticket_id::TicketId;
use async_trait::async_trait;
use std::any::Any;
use std::pin::Pin;

#[async_trait]
pub trait TicketRepository: Send + Sync {
    async fn find_by_id(&self, id: TicketId) -> Result<Ticket>;
    async fn insert(&mut self, ticket: Ticket) -> Result<()>;
    async fn save(&mut self, ticket: Ticket) -> Result<()>;
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    fn ticket_repo(&self) -> Box<dyn TicketRepository + '_>;
    async fn commit(self: Box<Self>) -> Result<()>;
}

#[async_trait]
pub trait UowFactory: Send + Sync {
    async fn execute_raw(&self, f: UowFnc) -> Result<Box<dyn Any + Send>>;
}

pub type UowFnc = Box<
    dyn FnOnce(
            Box<dyn UnitOfWork>,
        ) -> Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send>>> + Send>>
        + Send,
>;

#[async_trait]
pub trait UowFactoryExt: UowFactory {
    async fn execute_in_transaction<T, F, Fut>(&self, f: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce(Box<dyn UnitOfWork>) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T>> + Send,
    {
        let result = self
            .execute_raw(Box::new(|uow| {
                Box::pin(async move {
                    let res = f(uow).await?;
                    Ok(Box::new(res) as Box<dyn Any + Send>)
                })
            }))
            .await?;
        Ok(*result
            .downcast::<T>()
            .map_err(|_| DomainError::Infrastructure("Downcast failed".into()))?)
    }
}
impl<T: ?Sized + UowFactory> UowFactoryExt for T {}
