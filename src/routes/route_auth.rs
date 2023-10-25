use crate::controllers::controller_auth::login_user;
use crate::database::db::AppState;

use axum::{
    routing::{get, get_service, post},
    Router,
};
use std::sync::Arc;

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_user))
        .with_state(app_state)
}
