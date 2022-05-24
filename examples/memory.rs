fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}

fn main() {

    print!("{}", is_strong("123456"));
}
