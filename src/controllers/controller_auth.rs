use crate::database::db::AppState;
use crate::errors::errors::Error;
use crate::models::model_claim::Claims;
use crate::models::model_user::{AuthorizedUser, LoginUser, User};
use crate::utils::utils_auth::set_jwt_cookie;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

// @POST /api/auth/login
// @desc Login user
// @access Public
pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, HeaderMap, Json<AuthorizedUser>), Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| Error::LoginError("Invalid credentials.".to_string()))?;

    match user.verify_password(payload.password) {
        true => (),
        false => return Err(Error::LoginError("Invalid credentials.".to_string())),
        _ => return Err(Error::LoginError("Invalid credentials.".to_string())),
    }

    let authorized_user = AuthorizedUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    let response_headers = set_jwt_cookie(authorized_user.id.to_string())
        .await
        .unwrap();

    return Ok((StatusCode::OK, response_headers, Json(authorized_user)));
}
