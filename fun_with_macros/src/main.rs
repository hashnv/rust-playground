macro_rules! currency {
    ($num:expr) => {
        format!("£{:.2}", $num as f64)
    };
}

macro_rules! dumper {
    ($vect:expr) => {
        println!(
            "len={}, cap={}, cont={:?}",
            ($vect as Vec<_>).len(),
            ($vect as Vec<_>).capacity(),
            ($vect as Vec<_>),
        )
    };
}

fn main() {
    println!("You owe me {}", currency!(50));
    dumper!(vec!(1,2,3));
}
