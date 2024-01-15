use std::sync::Arc;

use warp::{Rejection, Reply};

use crate::driver::dto::{AddDevice, AddTable, AddTags, DelTags};
use crate::driver::mgr::DeviceMgr;

pub async fn get_drivers(device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&device_mgr.get_drivers()))
}

pub async fn get_devices(
    driver: Option<String>,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&device_mgr.get_devices(driver).await))
}

pub async fn del_device(name: String, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    device_mgr.del_device(&name).await?;
    Ok(warp::reply())
}

pub async fn add_device(
    device: AddDevice,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr
        .add_device(&device.name, &device.driver, &device.parameters)
        .await?;

    Ok(warp::reply())
}

pub async fn add_table(
    table: AddTable,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr
        .add_table(
            &table.device,
            &table.name,
            table.description,
            &table.parameters,
        )
        .await?;
    Ok(warp::reply())
}

pub async fn del_table(
    device: String,
    table: String,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr.del_table(&device, &table).await?;
    Ok(warp::reply())
}

pub async fn get_tables(
    device: String,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&device_mgr.get_tables(&device).await))
}

pub async fn add_tags(req: AddTags, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    device_mgr
        .add_tags(&req.device, &req.table, &req.tags)
        .await?;
    Ok(warp::reply())
}

pub async fn del_tags(req: DelTags, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    device_mgr
        .del_tags(&req.device, &req.table, &req.tags)
        .await?;
    Ok(warp::reply())
}

pub async fn get_tags(
    device: String,
    table: String,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(
        &device_mgr.get_tags(&device, &table, None).await,
    ))
}
