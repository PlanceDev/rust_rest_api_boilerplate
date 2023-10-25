use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    CreateUserError(String),
    GetUserError(String),
    LoginError(String),
    JwtError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::CreateUserError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }

            Error::GetUserError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }

            Error::LoginError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }

            Error::JwtError(message) => (StatusCode::UNAUTHORIZED, message).into_response(),
        }
    }
}
