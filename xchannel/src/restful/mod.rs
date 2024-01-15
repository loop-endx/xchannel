use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use warp::{http::Uri, Filter};

use crate::driver::mgr::DeviceMgr;

mod handler;
mod rejection;

#[derive(Debug)]
pub struct REST {
    pub host: SocketAddr,
}

impl REST {
    pub fn new(host: &str, port: u16) -> Result<REST, String> {
        let host = format!("{}:{}", host, port);
        let host = match host.parse() {
            Ok(host) => host,
            Err(_) => return Err(format!("Invalid host: {}", host)),
        };

        Ok(REST { host })
    }

    fn with_device_mgr(
        device_mgr: Arc<DeviceMgr>,
    ) -> impl Filter<Extract = (Arc<DeviceMgr>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || device_mgr.clone())
    }

    pub async fn serve(&self, device_mgr: Arc<DeviceMgr>) {
        let redirect_dashboard = warp::get()
            .and(warp::path::end())
            .map(|| warp::redirect(Uri::from_static("/web")));
        let dashboard = warp::get()
            .and(warp::path("web"))
            .and(warp::fs::dir("home"));

        let get_drivers = warp::get()
            .and(warp::path!("api" / "v1" / "driver"))
            .and(warp::path::end())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::get_drivers);

        let get_devices = warp::get()
            .and(warp::path!("api" / "v1" / "device"))
            .and(warp::query::<HashMap<String, String>>())
            .map(|query: HashMap<String, String>| query.get("driver").map(|x| x.to_string()))
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::get_devices);

        let add_device = warp::post()
            .and(warp::path!("api" / "v1" / "device"))
            .and(warp::body::json())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::add_device);

        let del_device = warp::delete()
            .and(warp::path!("api" / "v1" / "device" / String))
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::del_device);

        let add_tag_table = warp::post()
            .and(warp::path!("api" / "v1" / "device" / "table"))
            .and(warp::body::json())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::add_table);

        let del_tag_table = warp::delete()
            .and(warp::path!("api" / "v1" / "device" / "table"))
            .and(warp::path::param())
            .and(warp::path::param())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::del_table);

        let get_tag_tables = warp::get()
            .and(warp::path!("api" / "v1" / "device" / "table"))
            .and(warp::path::param())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::get_tables);

        let add_tags = warp::post()
            .and(warp::path!("api" / "v1" / "device" / "table" / "tag"))
            .and(warp::body::json())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::add_tags);

        let del_tags = warp::delete()
            .and(warp::path!("api" / "v1" / "device" / "table" / "tag"))
            .and(warp::body::json())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::del_tags);

        let get_tags = warp::get()
            .and(warp::path!("api" / "v1" / "device" / "table" / "tag"))
            .and(warp::path::param())
            .and(warp::path::param())
            .and(Self::with_device_mgr(device_mgr.clone()))
            .and_then(handler::get_tags);

        let routes = redirect_dashboard
            .or(dashboard)
            .or(get_drivers)
            .or(get_devices)
            .or(add_device)
            .or(del_device)
            .or(add_tag_table)
            .or(del_tag_table)
            .or(get_tag_tables)
            .or(add_tags)
            .or(del_tags)
            .or(get_tags)
            .recover(rejection::handle_rejection);
        warp::serve(routes).run(self.host).await;
    }
}
