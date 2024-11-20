use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{middleware::auth_mw::auth, AppState};

use super::{auth_handler::auth_handler, users_handler::users_handler};


pub fn api_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", auth_handler())
        .nest("/users", users_handler().layer(middleware::from_fn(auth)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
}
