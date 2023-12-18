mod drivers;
mod dto;
mod error;
mod manager;
mod module;
mod restful;

use crate::manager::driver;
use crate::manager::tag;

use restful::REST;


fn main() {
    let tags = tag::Tags::new();

    tags.add();

    let drivers = driver::Drivers::new();

    let x = REST::new("0.0.0.0", 7789);

    if let Ok(x) = x {
        x.serve(drivers, tags);
    } else {
        println!("Error: {:?}", x);
    }
}
