use std::sync::Arc;

use warp::{http::StatusCode, Rejection, Reply};

use crate::error::XError;
use crate::module::device_manager::DeviceMgr;

use super::request::{AddDevice, AddTable};
use super::response::{DelDevice, ErrorResponse, Response};

//pub async fn get_drivers(device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
//Ok(warp::reply::json(&device_mgr.get_drivers()))
//}

pub async fn get_devices(
    query: (Option<String>, Option<String>, Option<String>),
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    let devices = device_mgr.get_devices(query).await;

    Ok(Response::with_status(&devices, StatusCode::OK))
}

pub async fn del_device(name: String, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    if let Some(device) = device_mgr.del_device(&name).await? {
        Ok(Response::with_status(&DelDevice { device }, StatusCode::OK))
    } else {
        Ok(ErrorResponse::error(
            &XError::DeviceError(format!("{name} not found")),
            StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn add_device(
    device: AddDevice,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    let id = device_mgr
        .add_device(&device.name, &device.driver, &device.setting)
        .await?;

    Ok(Response::message(&id))
}

pub async fn get_tables(
    query: (String, Option<String>),
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    let tables = device_mgr.get_tables(&query.0, query.1).await?;
    Ok(Response::with_status(&tables, StatusCode::OK))
}

pub async fn add_table(
    device: String,
    table: AddTable,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr
        .add_table(&device, &table.name, table.description, &table.parameter)
        .await?;

    Ok(ErrorResponse::success())
}

//pub async fn del_table(
//device: String,
//table: String,
//device_mgr: Arc<DeviceMgr>,
//) -> Result<impl Reply, Rejection> {
//device_mgr.del_table(&device, &table).await?;
//Ok(warp::reply())
//}

//pub async fn add_tags(req: AddTags, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
//device_mgr
//.add_tags(&req.device, &req.table, &req.tags)
//.await?;
//Ok(warp::reply())
//}

//pub async fn del_tags(req: DelTags, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
//device_mgr
//.del_tags(&req.device, &req.table, &req.tags)
//.await?;
//Ok(warp::reply())
//}

//pub async fn get_tags(
//device: String,
//table: String,
//device_mgr: Arc<DeviceMgr>,
//) -> Result<impl Reply, Rejection> {
//Ok(warp::reply::json(
//&device_mgr.get_tags(&device, &table, None).await,
//))
//}
