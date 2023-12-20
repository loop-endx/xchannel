mod drivers;
mod dto;
mod error;
mod manager;
mod module;
mod restful;

use crate::manager::device;
use crate::manager::tag;

use restful::REST;

fn main() {
    let tags = tag::Tags::new();

    tags.add();

    let device_mgr = device::DeviceMgr::init();

    let x = REST::new("0.0.0.0", 7789);

    if let Ok(x) = x {
        x.serve(device_mgr, tags);
    } else {
        println!("Error: {:?}", x);
    }
}
