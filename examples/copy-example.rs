use std::marker::Copy;

// #[derive(Copy,Clone,Debug)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl Copy for CubeSat {}
impl Copy for StatusMessage {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        *self
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

// #[derive(Copy,Clone,Debug)]
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}



fn main() {
    let sat_a = CubeSat{id:0};
    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
