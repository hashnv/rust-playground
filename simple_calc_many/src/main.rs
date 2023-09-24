use std::process;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 3 {
        eprintln!("Not enough args supplied!");
        process::exit(1);
    }

    let (numbers, _errors): (Vec<_>, Vec<_>) = argv[1..]
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let num_sum: i32 = numbers.into_iter().map(Result::unwrap).sum();

    println!("{}", num_sum);
}
