use std::process;

pub fn is_palindrome(s: &String) -> bool {
    let reversed_s = s.chars().rev().collect::<String>();
    *s == reversed_s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_easy() {
        assert!(is_palindrome(&String::from("racecar")))
    }

    #[test]
    fn test_is_palindrome_medium() {
        assert!(is_palindrome(&String::from("A nut for a jar of tuna")))
    }

    #[test]
    fn test_is_palindrome_hard() {
        assert!(is_palindrome(&String::from("A man, a plan, a canal, Panama!")))
    }

    #[test]
    fn test_is_palindrome_failure() {
        assert_eq!(is_palindrome(&String::from("racingcar")), false)
    }
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
