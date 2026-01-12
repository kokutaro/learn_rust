mod ticket_handler;

use crate::presentation::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tickets", post(ticket_handler::create_ticket))
        .route("/tickets/{id}", delete(ticket_handler::close_ticket))
}
