fn main() {
    let mut str_input = String::new();
    std::io::stdin()
        .read_line(&mut str_input)
        .expect("Unable to read from stdin");
    let str_trimmed = str_input.trim();
    match str_trimmed.parse::<u16>() {
        Ok(i) => println!("Integer plus 1: {}", i + 1),
        Err(..) => println!("Not an integer: {}", str_trimmed),
    }
}
