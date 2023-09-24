pub fn has_min_length(s: &str, min_length: usize) -> bool {
    s.chars().count() >= min_length
}

pub fn contains_uppercase(s: &str) -> bool {
    s.chars().any(|x| x.is_uppercase())
}

pub fn contains_digit(s: &str) -> bool {
    s.chars().any(|x| x.is_ascii_digit())
}

pub fn contains_punctuation(s: &str) -> bool {
    s.chars().any(|x| !x.is_alphanumeric())
}

pub fn valid_password(password: &str) -> Result<&str, &'static str> {
    if !has_min_length(password, 8) {
        Err("Less than eight characters long.")
    } else if !contains_uppercase(password) {
        Err("Does not contain an uppercase character.")
    } else if !contains_digit(password) {
        Err("Does not contain a digit.")
    } else if !contains_punctuation(password) {
        Err("Does not contain punctuation.")
    } else {
        Ok(password)
    }
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

    match valid_password(&passwd) {
        Ok(_) => eprintln!("That is a valid password! :)"),
        Err(e) => eprintln!("InvalidPasswordError: '{}'", e)
    }
}
