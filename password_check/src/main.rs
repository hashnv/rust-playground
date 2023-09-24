pub fn has_min_length(s: &str, min_length: usize) -> bool {
    s.chars().count() >= min_length
}

pub fn contains_uppercase(s: &str) -> bool {
    s.chars().find(|x| x.is_uppercase()).is_some()
}

pub fn contains_digit(s: &str) -> bool {
    s.chars().find(|x| x.is_digit(10)).is_some()
}

pub fn contains_punctuation(s: &str) -> bool {
    s.chars().find(|x| !x.is_alphanumeric()).is_some()
}

pub fn is_valid_password(s: &str) -> bool {
    has_min_length(s, 8) && contains_uppercase(s) && contains_digit(s) && contains_punctuation(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_min_length() {
        assert!(has_min_length("albatross", 8));
    }

    #[test]
    fn test_not_has_min_length() {
        assert!(!has_min_length("monkey", 8));
    }

    #[test]
    fn test_is_min_length() {
        assert!(has_min_length("elephant", 8));
    }

    #[test]
    fn test_contains_uppercase() {
        assert!(contains_uppercase("Elephant"));
    }

    #[test]
    fn test_not_contains_uppercase() {
        assert!(!contains_uppercase("albatross"));
    }

    #[test]
    fn test_contains_digit() {
        assert!(contains_digit("elephant1"));
    }

    #[test]
    fn test_not_contains_digit() {
        assert!(!contains_digit("albatross"));
    }

    #[test]
    fn test_contains_punctuation() {
        assert!(contains_punctuation("password!"))
    }

    #[test]
    fn test_not_contains_punctuation() {
        assert!(!contains_punctuation("password"))
    }
}

fn main() {
    eprint!("You'll need a password: ");

    let mut passwd = String::new();
    std::io::stdin()
        .read_line(&mut passwd)
        .expect("Unable to read from stdin");

    match is_valid_password(&passwd) {
        true => println!("Your password is valid."),
        false => println!("Your password is NOT valid."),
    }
}
