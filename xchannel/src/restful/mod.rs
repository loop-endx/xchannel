use std::net::SocketAddr;

use tokio;
use warp::{http::Uri, Filter};

use crate::driver::drivers::Drivers;

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

    pub fn serve(&self) -> () {
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
                .and(warp::path("api"))
                .and(warp::path("drivers"))
                .map(|| {
                    let drivers = Drivers::new();
                    warp::reply::json(&drivers.get_drivers())
                });

            let routes = redirect_dashboard.or(dashboard).or(get_drivers);
            warp::serve(routes).run(self.host).await;
        })
    }
}
