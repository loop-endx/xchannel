use serde::Serialize;
use warp::{
    http::StatusCode,
    reply,
    reply::{Json, WithStatus},
};

use crate::error::XError;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse<'a> {
    code: i64,
    message: &'a str,
}

impl<'a> ErrorResponse<'a> {
    pub fn success() -> WithStatus<Json> {
        Self::response(0, "success", StatusCode::OK)
    }

    pub fn response(code: i64, message: &str, status: StatusCode) -> WithStatus<Json> {
        reply::with_status(reply::json(&ErrorResponse { code, message }), status)
    }

    pub fn error(err: &XError, status: StatusCode) -> WithStatus<Json> {
        reply::with_status(
            reply::json(&ErrorResponse {
                code: err.code(),
                message: &err.to_string(),
            }),
            status,
        )
    }
}

pub struct Response;

impl Response {
    pub fn with_status<T: Serialize>(body: &T, status: StatusCode) -> WithStatus<Json> {
        reply::with_status(reply::json(body), status)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DelDevice<'a> {
    pub device: &'a str,
}
