use std::net::SocketAddr;
use std::sync::Arc;

use tokio;
use warp::{http::Uri, Filter};

use crate::manager::{device::DeviceMgr, tag::Tags};

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

    pub fn serve(&self, device_mgr: Arc<DeviceMgr>, tags_mgr: Arc<Tags>) -> () {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let redirect_dashboard = warp::get()
                .and(warp::path::end())
                .map(|| warp::redirect(Uri::from_static("/web")));
            let dashboard = warp::get()
                .and(warp::path("web"))
                .and(warp::fs::dir("home"));

            let get_drivers = warp::get()
                .and(warp::path!("api" / "driver"))
                .and(Self::with_device_mgr(device_mgr.clone()))
                .and_then(handler::get_drivers);

            let get_devices = warp::get()
                .and(warp::path!("api" / "device"))
                .and(Self::with_device_mgr(device_mgr.clone()))
                .and_then(handler::get_devices);

            let add_device = warp::post()
                .and(warp::path!("api" / "device"))
                .and(warp::body::json())
                .and(Self::with_device_mgr(device_mgr.clone()))
                .and_then(handler::add_device);

            let del_device = warp::delete()
                .and(warp::path!("api" / "device"))
                .and(warp::path::param())
                .and(Self::with_device_mgr(device_mgr.clone()))
                .and_then(handler::del_device);

            //let add_tags = warp::post()
            //.and(warp::path("api"))
            //.and(warp::path("tag"))
            //.and(warp::body::json())
            //.and(warp::any().map(move || tags_mgr.clone()))
            //.and_then(handler::add_tags);

            let routes = redirect_dashboard
                .or(dashboard)
                .or(get_drivers)
                .or(get_devices)
                .or(add_device)
                .or(del_device)
                // .or(add_tags)
                .recover(rejection::handle_rejection);
            warp::serve(routes).run(self.host).await;
        })
    }
}
