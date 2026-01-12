use crate::infrastructure::repository::sqlx_ticket_repository::SqlxUowFactory;
use crate::presentation::{http, AppState};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::log::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json().compact())
        .init();
    info!("test");

    let service = AppState {
        uow_factory: Arc::new(SqlxUowFactory::new(pool)),
    };
    let app: Router = http::router().with_state(service);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
