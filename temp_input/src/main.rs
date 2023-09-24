use std::io;

fn celsius_to_fahrenheit(num: f64) -> f64 {
    (num * 1.8) + 32.0
}
fn main() {
    let mut buffer = String::new();
    eprint!("Please enter a tempurature in C: ");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Unable to read from stdin");
    let temp = buffer.trim().parse::<f64>();
    match temp {
        Ok(i) => println!("Temp is {} F", celsius_to_fahrenheit(i)),
        Err(..) => eprintln!("ruh roh, that's not a temp!"),
    };
}
