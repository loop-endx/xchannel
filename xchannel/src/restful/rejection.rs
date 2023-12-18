use std::convert::Infallible;

use serde_derive::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::{self, WithStatus}, Rejection, Reply};

use crate::error::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ErrorResponse {
    code: u64,
    message: String,
}

fn default_rejection(err: Rejection) -> Result<WithStatus<warp::reply::Json>, Infallible> {
    if let Some(err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(reply::with_status(
            reply::json(&ErrorResponse {
                code: 0xFFFFFFFF,
                message: err.to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(err) = err.find::<warp::reject::UnsupportedMediaType>() {
        Ok(reply::with_status(
            reply::json(&ErrorResponse {
                code: 0xFFFFFFFF,
                message: err.to_string(),
            }),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(err) = err.find::<warp::reject::MethodNotAllowed>() {
        Ok(reply::with_status(
            reply::json(&ErrorResponse {
                code: 0xFFFFFFFF,
                message: err.to_string(),
            }),
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else {
        Ok(reply::with_status(
            reply::json(&ErrorResponse {
                code: 0xFFFFFFFF,
                message: "Unhandled Error".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(reply::with_status(
            reply::json(&ErrorResponse {
                code: 0xFFFFFFFF,
                message: "Not Found".to_string(),
            }),
            StatusCode::NOT_FOUND,
        ));
    }

    if let Some(e) = err.find::<XError>() {
        let (status, body) = match e {
            _ => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: e.code(),
                    message: e.message(),
                },
            ),
        };

        Ok(reply::with_status(reply::json(&body), status))
    } else {
        default_rejection(err)
    }
}
