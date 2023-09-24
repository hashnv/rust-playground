use std::process::exit;

pub fn has_double(s: &str) -> bool {
    let s = s.to_lowercase();
    let words = s.split(' ');
    let mut previous_char;
    for word in words {
        previous_char = ' ';
        for char in word.chars() {
            if char == previous_char {
                return true;
            }
            previous_char = char;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_double() {
        assert!(has_double("hello"))
    }

    #[test]
    fn test_has_double_failure() {
        assert!(!has_double("help"))
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 2 {
        eprintln!("Not enough args supplied!");
        exit(1);
    }

    let s = &argv[1];

    match has_double(s) {
        true => println!("'{}' contains a double letter.", s),
        _ => println!("'{}' does NOT contain a double letter.", s),
    }
}
