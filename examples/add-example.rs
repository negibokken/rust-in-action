use detail_rust::add::add;
use std::time::{Duration};

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10,20);
    let duration = add(
        Duration::new(5, 0),
        Duration::new(10, 0),
    );
    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", duration);
}
