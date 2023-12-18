use std::sync::Arc;

use warp::{http::StatusCode, reject, Rejection, Reply};

use crate::dto;
use crate::error::*;
use crate::manager::{driver::Drivers, tag::Tags};

pub async fn get_drivers(drivers: Arc<Drivers>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&drivers.get_drivers()))
}

pub async fn test_error(drivers: Arc<Drivers>) -> Result<impl Reply, Rejection> {
    let a: Result<i32, String> = Err("h1".to_string());

    a.map_err(|_| reject::custom(TagError::new(TagErrorKind::Invalid, "llllll")))?;

    Ok(warp::reply::json(&drivers.get_drivers()))
}

pub async fn add_tags(
    tags: Vec<dto::tag::Tag>,
    tags_mgr: Arc<Tags>,
) -> Result<impl Reply, Rejection> {
    dto::tag::Tag::check(&tags)?;
    tags_mgr.add();
    tags_mgr.insert("modbus".to_string(), tags)?;

    //println!("{:?}", tags);
    //Ok(warp::reply::with_status("", StatusCode::CREATED))
    Ok(StatusCode::CREATED)
}
