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

#[derive(Debug, Clone, Serialize)]
struct ResponseMsg<'a> {
    index: i32,
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

    pub fn message(msg: &str) -> WithStatus<Json> {
        Self::response(msg, StatusCode::OK)
    }

    pub fn partial(index: i32, msg:&str) -> WithStatus<Json> {
        reply::with_status(reply::json(&ResponseMsg { index, message: msg }), StatusCode::PARTIAL_CONTENT)
    }

    pub fn response(message: &str, status: StatusCode) -> WithStatus<Json> {
        reply::with_status(reply::json(&ResponseMsg { index: 0, message }), status)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DelDevice<'a> {
    pub device: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DelTable<'a> {
    pub table: &'a str,
}
