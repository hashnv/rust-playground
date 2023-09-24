macro_rules! currency {
    ($num:expr) => {
        format!("£{:.2}", $num as f64)
    };
}

macro_rules! dumper {
    ($($item:expr),*) => {
        let temp_vec = vec!($($item),*);
        println!(
            "len={}, cap={}, cont={:?}",
            temp_vec.len(),
            temp_vec.capacity(),
            temp_vec,
        )
    };
}

fn main() {
    println!("You owe me {}", currency!(50));
    dumper!(1,2,3,4,5,6,7,8);
}
