use std::sync::Arc;

use warp::{http::StatusCode, Rejection, Reply};

use crate::error::*;
use crate::module::device_manager::DeviceMgr;
use crate::module::tag::Tag;

use super::request::{AddDevice, AddTable, AddTag, DelTag};
use super::response::{DelDevice, DelTable, ErrorResponse, Response};

pub async fn get_drivers(device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    let drivers = device_mgr.get_drivers();

    Ok(Response::with_status(&drivers, StatusCode::OK))
}

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

pub async fn del_table(
    device: String,
    table: String,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    if let Some(name) = device_mgr.del_table(&device, &table).await? {
        Ok(Response::with_status(
            &DelTable { table: name },
            StatusCode::OK,
        ))
    } else {
        Ok(ErrorResponse::error(
            &XError::TableError(format!("{table} not found")),
            StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn add_tags(
    device: String,
    table: String,
    tags: Vec<AddTag>,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    let tags = tags
        .iter()
        .enumerate()
        .map(|(index, tag)| {
            let tag: XResult<Tag> = tag.try_into();
            if let Err(e) = tag {
                Err(e.with_index(index as i32))
            } else {
                tag
            }
        })
        .collect::<XResult<Vec<Tag>>>()?;
    if let Err(e) = device_mgr.add_tags(&device, &table, tags).await {
        let index = e.get_index();
        Ok(Response::partial(index, &e.to_string()))
    } else {
        Ok(Response::message("success"))
    }
}

pub async fn del_tags(
    device: String,
    table: String,
    tags: Vec<DelTag>,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr
        .del_tags(
            &device,
            &table,
            tags.iter().map(|t| t.name.clone()).collect::<Vec<String>>(),
        )
        .await?;
    Ok(ErrorResponse::success())
}

pub async fn get_tags(
    query: (String, String, Option<String>),
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    let tags = device_mgr.get_tags(&query.0, &query.1, query.2).await?;

    Ok(Response::with_status(&tags, StatusCode::OK))
}
