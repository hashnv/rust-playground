fn add_it(n1: u8, n2: u8) -> Result<u8, &'static str> {
    match n1.checked_add(n2) {
        Some(x) => Ok(x),
        None => Err("addition error"),
    }
}

fn main() {
    let num1 = 254;
    let num2 = 254;

    println!("{:?}", add_it(num1, num2))
}
