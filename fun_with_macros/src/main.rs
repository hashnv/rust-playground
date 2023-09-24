macro_rules! currency {
    ($num:expr) => {
        format!("£{:.2}", $num as f64)
    };
}

fn main() {
    println!("You owe me {}", currency!(50))
}
