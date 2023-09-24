use std::process::exit;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 3 {
        eprintln!("Not enough args supplied!");
        exit(1);
    }

    let num1_result = argv[1].parse::<i32>();
    let num1 = match num1_result {
        Ok(i) => i,
        Err(..) => panic!("First arg must be an int"),
    };

    let num2_result = argv[2].parse::<i32>();
    let num2 = match num2_result {
        Ok(i) => i,
        Err(..) => panic!("Second arg must be an int"),
    };

    println!("{} + {} = {}", num1, num2, num1 + num2)
}
