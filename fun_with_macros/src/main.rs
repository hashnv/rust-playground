macro_rules! currency {
    ($num:expr) => {
        format!("£{:.2}", $num as f64)
    };
}

macro_rules! dumper {
    ($vect:expr) => {
        let v: Vec<_> = $vect as Vec<_>;
        let v_len = v.len();
        let v_cap = v.capacity();
        println!("len={v_len}, cap={v_cap}, cont={v:?}")
    };
}

fn main() {
    println!("You owe me {}", currency!(50));
    let my_vec: Vec<_> = vec!(1,2,3);
    dumper!(my_vec);
}
