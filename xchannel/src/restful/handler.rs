use std::sync::Arc;

use warp::{Rejection, Reply};

use crate::{dto::device::AddDevice, manager::device::DeviceMgr};

pub async fn get_drivers(device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&device_mgr.get_driver_info()))
}

pub async fn get_devices(device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&device_mgr.get_device()))
}

pub async fn del_device(name: String, device_mgr: Arc<DeviceMgr>) -> Result<impl Reply, Rejection> {
    device_mgr.del_device(&name)?;
    Ok(warp::reply())
}

pub async fn add_device(
    device: AddDevice,
    device_mgr: Arc<DeviceMgr>,
) -> Result<impl Reply, Rejection> {
    device_mgr.add_device(&device.name, &device.driver)?;
    Ok(warp::reply())
}

//pub async fn add_tags(
//tags: Vec<dto::tag::Tag>,
//tags_mgr: Arc<Tags>,
//) -> Result<impl Reply, Rejection> {
//dto::tag::Tag::check(&tags)?;
//tags_mgr.add();
//tags_mgr.insert("modbus".to_string(), tags)?;

////println!("{:?}", tags);
////Ok(warp::reply::with_status("", StatusCode::CREATED))
//Ok(StatusCode::CREATED)
//}
