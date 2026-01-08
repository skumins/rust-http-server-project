use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

pub enum AppError {
    InvalidData(String),
    DatabaseError(String),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Record not found".to_string()),
        };
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}