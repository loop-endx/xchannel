use tracing::{error, info, level_filters::LevelFilter};
use tracing_appender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

//mod db;
//mod driver;
mod drivers;
mod error;
mod restful;
//mod tag;

mod module;

//use driver::mgr;
use module::device_manager::DeviceMgr;

use restful::REST;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "xchannel.log");
    let (nb, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(nb)
        .with_filter(LevelFilter::INFO);
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(LevelFilter::INFO);

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    let device_mgr = DeviceMgr::init().await;
    if let Err(e) = device_mgr {
        error!("Error: {:?}", e);
        return;
    }
    let device_mgr = device_mgr.unwrap();

    let x = REST::new("0.0.0.0", 5260);

    if let Ok(x) = x {
        info!("xchannel started.");
        x.serve(device_mgr).await;
    } else {
        error!("Error: {:?}", x);
    }
}
