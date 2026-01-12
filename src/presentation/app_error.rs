use crate::domain::error::DomainError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DomainError::Ticket(ticket_error) => {
                (StatusCode::BAD_REQUEST, ticket_error.to_string())
            }
            DomainError::RepositoryError(repository_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, repository_error)
            }
            DomainError::ConcurrentModification => (StatusCode::CONFLICT, self.to_string()),
            DomainError::Infrastructure(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DomainError::InvalidTicketId => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
