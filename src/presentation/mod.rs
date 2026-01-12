use crate::domain::tickets::repository::UowFactory;
use axum::extract::FromRef;
use std::sync::Arc;

pub mod http;
mod app_error;

#[derive(Clone)]
pub struct AppState {
    pub uow_factory: Arc<dyn UowFactory>,
}

impl FromRef<AppState> for Arc<dyn UowFactory> {
    fn from_ref(input: &AppState) -> Self {
        input.uow_factory.clone()
    }
}
