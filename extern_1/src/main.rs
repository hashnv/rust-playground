extern "C" {
    fn sqrt(val: f64) -> f64;
}

fn main() {
    let number: f64 = 2.0;
    let sq_root: f64;

    sq_root = unsafe { sqrt(number) };

    println!("Sqrt is {}", sq_root)
}
