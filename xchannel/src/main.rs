use tracing::{error, info};
use tracing_appender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};

mod driver;
mod drivers;
mod error;
mod restful;
mod tag;

use driver::mgr;

use restful::REST;

fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "xchannel.log");
    let (nb, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_ansi(false).with_writer(nb);
    let stdout_layer = fmt::layer().with_writer(std::io::stdout);

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    let device_mgr = mgr::DeviceMgr::init();

    let x = REST::new("0.0.0.0", 5260);

    if let Ok(x) = x {
        info!("xchannel started.");
        x.serve(device_mgr);
    } else {
        error!("Error: {:?}", x);
    }
}
