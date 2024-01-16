use std::convert::Infallible;

use warp::{http::StatusCode, reply::WithStatus, Rejection, Reply};

use super::response::ErrorResponse;
use crate::error::*;

fn default_rejection(err: Rejection) -> Result<WithStatus<warp::reply::Json>, Infallible> {
    if let Some(err) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(ErrorResponse::response(
            -1,
            &err.to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(err) = err.find::<warp::reject::UnsupportedMediaType>() {
        Ok(ErrorResponse::response(
            -1,
            &err.to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else if let Some(err) = err.find::<warp::reject::MethodNotAllowed>() {
        Ok(ErrorResponse::response(
            -1,
            &err.to_string(),
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else {
        Ok(ErrorResponse::response(
            -1,
            "Unhandled Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(ErrorResponse::response(
            -1,
            "Not Found",
            StatusCode::NOT_FOUND,
        ));
    }

    if let Some(e) = err.find::<XError>() {
        Ok(ErrorResponse::response(
            e.code(),
            &e.message(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        default_rejection(err)
    }
}
