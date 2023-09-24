use std::process;

fn is_palindrome(s: &String) -> bool {
    let reversed_s = s.chars().rev().collect::<String>();
    *s == reversed_s
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 2 {
        eprintln!("Not enough args supplied!");
        process::exit(1);
    }

    let strings = &argv[1..];

    for s in strings {

        match is_palindrome(s) {
            true => println!("{} is a palindrome.", s),
            _ => println!("{} is NOT a palindrome.", s),
        }
    }
}
