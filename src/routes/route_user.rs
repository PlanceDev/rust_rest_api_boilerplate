use crate::{
    controllers::controller_user::{create_user, get_user_by_id, get_users},
    database::db::AppState,
};

use axum::{
    routing::{get, get_service, post},
    Router,
};
use std::sync::Arc;

pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users", get(get_users))
        .route("/api/users/:id", get(get_user_by_id))
        .with_state(app_state)
}
