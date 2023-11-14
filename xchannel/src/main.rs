mod restful;
mod device;

use restful::REST;

fn main() {
    let x = REST::new("0.0.0.0", 7789);

    if let Ok(x) = x {
        x.serve();
    } else {
        println!("Error: {:?}", x);
    }

}
