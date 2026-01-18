use crate::infrastructure::repository::sqlx_ticket_repository::SqlxUowFactory;
use crate::presentation::{http, AppState};
use axum::Router;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::log;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // OTLP Exporter setting
    let endpoint = "http://localhost:4317";

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .build();

    let tracer = tracer_provider.tracer("learn-rust");
    let telemetry_layer =
        tracing_opentelemetry::layer::<tracing_subscriber::Registry>().with_tracer(tracer);

    let logger_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let logger_provider = opentelemetry_sdk::logs::LoggerProviderBuilder::default()
        .with_batch_exporter(logger_exporter)
        .build();

    let logger_layer =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider);

    let filter = EnvFilter::new("off").add_directive("learn_rust=trace".parse()?);

    tracing_subscriber::registry()
        .with(telemetry_layer.with_filter(filter.clone()))
        .with(logger_layer.with_filter(filter.clone()))
        .with(tracing_subscriber::fmt::layer().with_filter(filter.clone()))
        .init();

    log::info!("Application started successfully");

    let service = AppState {
        uow_factory: Arc::new(SqlxUowFactory::new(pool)),
    };
    let app: Router = http::router().with_state(service);

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
