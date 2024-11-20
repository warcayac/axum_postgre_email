pub mod api;

use std::sync::Arc;

use axum::Router;
use api::routes::api_route;

use crate::AppState;


pub fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api", api_route(app_state))
}