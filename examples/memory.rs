fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}

fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);
    print!("{}", is_strong("123456"));
    print!("a: {}, b: {}, a+b: {}", a, b, a+*b);
}
