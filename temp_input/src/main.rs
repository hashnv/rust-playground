use std::io;

fn celsius_to_fahrenheit(num: f64) -> f64 {
    (num * 1.8) + 32.0
}
fn main() {
    eprint!("Please enter a tempurature in C: ");
    
    let mut buffer = String::new();
    let line = io::stdin().read_line(&mut buffer);
    match line {
        Ok(_i) => (),
        Err(..) => eprintln!("ruh roh, couldn't read stdin!"),
    };

    let temp = buffer.trim().parse::<f64>();
    match temp {
        Ok(i) => println!("Temp is {} F", celsius_to_fahrenheit(i)),
        Err(..) => eprintln!("ruh roh, that's not a temp!"),
    };
}
