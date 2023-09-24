use std::process;

pub fn sanitize(s: &String) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    s.to_lowercase().chars().filter(|c| alphabet.contains(*c)).collect()
}

pub fn is_palindrome(s: &String) -> bool {
    let s = sanitize(s);
    let reversed_s = s.chars().rev().collect::<String>();
    return s == reversed_s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_easy() {
        assert!(is_palindrome(&String::from("toot")))
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
        assert!(!is_palindrome(&String::from("racingcar")))
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
            true => println!("'{}' is a palindrome.", s),
            _ => println!("'{}' is NOT a palindrome.", s),
        }
    }
}
