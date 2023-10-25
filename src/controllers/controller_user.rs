use crate::database::db::AppState;
use crate::errors::errors::Error;
use crate::models::model_user::{CreateUser, GetUserParams, SearchUser, User};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

// @POST /api/users
// @desc Create user
// @access Public
pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), Error> {
    let user = User::new(
        payload.username,
        payload.display_name,
        payload.email,
        payload.password,
    );

    let mut session = app_state
        .clone()
        .db
        .begin()
        .await
        .map_err(|e| Error::CreateUserError("Database connection failed.".to_string()))?;

    match user.save(&mut session).await {
        Ok(_) => (),
        Err(e) => {
            session.rollback().await.unwrap();
            return Err(Error::CreateUserError(
                "Could not create account, please try again later.".to_string(),
            ));
        }
    }

    session.commit().await.unwrap();
    Ok((StatusCode::CREATED, Json(user)))
}

// @GET /api/users
// @desc Get users by query
// @access Public
pub async fn get_users(
    Query(params): Query<GetUserParams>,
    State(app_state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<SearchUser>>), Error> {
    let limit = params.limit.unwrap_or(100).min(100);
    let offset = params.offset.unwrap_or(0);

    let users = sqlx::query_as::<_, SearchUser>(
        "SELECT id, username, display_name, created_at, updated_at FROM users LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| Error::GetUserError("Could not find users.".to_string()))?;

    Ok((StatusCode::OK, Json(users)))
}

// @GET /api/users/:id
// @desc Get a user by id
// @access Public
pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<SearchUser>), Error> {
    let user = sqlx::query_as::<_, SearchUser>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|e| Error::GetUserError("User not found.".to_string()))?;

    Ok((StatusCode::OK, Json(user)))
}
