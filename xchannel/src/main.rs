mod restful;

use restful::REST;

fn main() {
    let x = REST::new("0.0.0.0", 7789);

    println!("Hello, world! {:?}", x);
}
