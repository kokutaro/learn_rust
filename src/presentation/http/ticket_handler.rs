use crate::application::usecase;
use crate::domain::tickets::repository::UowFactory;
use crate::presentation::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_ticket(
    State(uow_factory): State<Arc<dyn UowFactory>>,
    Json(request): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    let id = usecase::tickets::create_ticket(
        uow_factory.as_ref(),
        request.title.to_string(),
        request.description.to_string(),
    )
    .await;
    match id {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            error!("{:?}", e);
            e.into_response()
        }
    };
}

pub async fn close_ticket(
    State(service): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    usecase::tickets::close_ticket(service.uow_factory.as_ref(), id).await
}
