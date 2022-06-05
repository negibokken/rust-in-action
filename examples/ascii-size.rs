fn main() {
    let a = "AAABBCDDDD";
    let mut bytes = "".to_string();
    for c in a.clone().bytes() {
        bytes += &format!("{:b}\n",c);
    }
    println!("{}",bytes.len());
    let c = "1010101101101110000";
    println!("{}",c.len());
 }
