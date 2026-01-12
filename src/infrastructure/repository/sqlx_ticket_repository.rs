use crate::domain::error::{DomainError, Result};
use crate::domain::tickets::repository::{TicketRepository, UnitOfWork, UowFactory, UowFnc};
use crate::domain::tickets::ticket::{Ticket, TicketId};
use crate::domain::tickets::ticket_status::TicketStatus;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info_span, Instrument};
use uuid::Uuid;

pub struct SqlxUowFactory {
    pool: sqlx::PgPool,
}

impl SqlxUowFactory {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UowFactory for SqlxUowFactory {
    async fn execute_raw(&self, f: UowFnc) -> Result<Box<dyn Any + Send>> {
        let span = info_span!("db_transaction");

        // 1. Begin transaction
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DomainError::Infrastructure(e.into()))?;
        let tx_shared = Arc::new(Mutex::new(tx));
        let uow = Box::new(SqlxUnitOfWork {
            tx: tx_shared.clone(),
        });

        // 2. Execute closure(use case Logic)
        let result = f(uow).instrument(span).await;

        match result {
            Ok(value) => {
                // 3. Commit transaction
                // Assume that transaction reference is not leaked
                if let Ok(tx_mutex) = Arc::try_unwrap(tx_shared) {
                    let tx = tx_mutex.into_inner();
                    tx.commit()
                        .await
                        .map_err(|e| DomainError::Infrastructure(e.into()))?;
                    Ok(value)
                } else {
                    Err(DomainError::Infrastructure(
                        "Transaction reference leak".into(),
                    ))
                }
            }
            // 4. Rollback transaction on error
            // Automatically rollbacks when the transaction is dropped(sqlx feature)
            Err(e) => Err(e),
        }
    }
}

pub struct SqlxTicketRepository<'a> {
    tx: &'a Mutex<Transaction<'static, Postgres>>,
}

pub struct SqlxUnitOfWork {
    tx: Arc<Mutex<Transaction<'static, Postgres>>>,
}

#[async_trait]
impl UnitOfWork for SqlxUnitOfWork {
    fn ticket_repo(&self) -> Box<dyn TicketRepository + '_> {
        Box::new(SqlxTicketRepository { tx: &self.tx })
    }

    async fn commit(self: Box<Self>) -> Result<()> {
        let tx_mutex = Arc::into_inner(self.tx).unwrap();
        let tx = tx_mutex.into_inner();
        tx.commit()
            .await
            .map_err(|e| DomainError::Infrastructure(e.into()))?;
        Ok(())
    }
}

#[async_trait]
impl<'a> TicketRepository for SqlxTicketRepository<'a> {
    async fn find_by_id(&self, id: TicketId) -> Option<Ticket> {
        let mut tx = self.tx.lock().await;
        let row = sqlx::query_as!(
            TicketRow,
            r#"
            SELECT id, title, description, status, assignee, version
            FROM tickets
            WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))
        .ok()??;

        Some(Ticket::reconstruct(
            TicketId::from(row.id),
            row.title,
            row.description,
            row.status.parse().ok()?,
            row.assignee,
            row.version,
        ))
    }
    async fn insert(&mut self, ticket: Ticket) -> Result<()> {
        let mut tx = self.tx.lock().await;
        let status = ticket.status().to_string();

        sqlx::query!(
            r#"
            INSERT INTO tickets
            (id, title, description, status, assignee, version)
            VALUES ($1, $2, $3, $4, $5, 0)
            "#,
            ticket.id().value(),
            ticket.title(),
            ticket.description(),
            status,
            ticket.assignee(),
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        Ok(())
    }

    async fn save(&mut self, ticket: Ticket) -> Result<()> {
        let mut tx = self.tx.lock().await;
        let status = match ticket.status() {
            TicketStatus::Open => "open",
            TicketStatus::Assigned { user_id: _ } => "assigned",
            TicketStatus::Closed => "closed",
        };

        let result = sqlx::query!(
            r#"
            UPDATE tickets
            SET
                title = $1,
                description = $2,
                status = $3,
                assignee = $4,
                version = version + 1
            WHERE id = $5 AND version = $6
            "#,
            ticket.title(),
            ticket.description(),
            status,
            ticket.assignee(),
            ticket.id().value(),
            ticket.version(),
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::ConcurrentModification);
        }
        Ok(())
    }
}

struct TicketRow {
    id: Uuid,
    title: String,
    description: String,
    status: String,
    assignee: Option<Uuid>,
    version: i64,
}
