mod drivers;
mod error;
mod restful;

mod driver;
mod tag;

use driver::mgr;

use restful::REST;

fn main() {
    //    let tags = tag::Tags::new();

    //tags.add();

    let device_mgr = mgr::DeviceMgr::init();

    let x = REST::new("0.0.0.0", 5260);

    if let Ok(x) = x {
        x.serve(device_mgr);
    } else {
        println!("Error: {:?}", x);
    }
}
