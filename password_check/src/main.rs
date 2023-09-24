pub fn has_min_length_eight(s: &str) -> Result<(), &'static str> {
    match s.chars().count() >= 8 {
        true => Ok(()),
        false => Err("Less than eight characters long."),
    }
}

pub fn contains_uppercase(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| x.is_uppercase()) {
        true => Ok(()),
        false => Err("Does not contain an uppercase character."),
    }
}

pub fn contains_digit(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| x.is_ascii_digit()) {
        true => Ok(()),
        false => Err("Does not contain a digit."),
    }
}

pub fn contains_punctuation(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| !x.is_alphanumeric()) {
        true => Ok(()),
        false => Err("Does not contain punctuation."),
    }
}

const PASSWORD_CHECKS: &'static [fn(&str) -> Result<(), &'static str>] = &[
    has_min_length_eight,
    contains_uppercase,
    contains_digit,
    contains_punctuation,
];

pub fn valid_password(password: &str) -> Result<(), &'static str> {
    let results: &[Option<Result<(), &'static str>>; PASSWORD_CHECKS.len()] =
        &[None; PASSWORD_CHECKS.len()];
    for (i, check) in PASSWORD_CHECKS.iter().enumerate() {
        results[i] = check(password);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_min_length_eight() {
        assert!(has_min_length_eight("albatross").is_ok());
    }

    #[test]
    fn test_not_has_min_length_eight() {
        assert!(!has_min_length_eight("monkey").is_ok());
    }

    #[test]
    fn test_is_min_length() {
        assert!(has_min_length_eight("elephant").is_ok());
    }

    #[test]
    fn test_contains_uppercase() {
        assert!(contains_uppercase("Elephant").is_ok());
    }

    #[test]
    fn test_not_contains_uppercase() {
        assert!(!contains_uppercase("albatross").is_ok());
    }

    #[test]
    fn test_contains_digit() {
        assert!(contains_digit("elephant1").is_ok());
    }

    #[test]
    fn test_not_contains_digit() {
        assert!(!contains_digit("albatross").is_ok());
    }

    #[test]
    fn test_contains_punctuation() {
        assert!(contains_punctuation("password!").is_ok())
    }

    #[test]
    fn test_not_contains_punctuation() {
        assert!(!contains_punctuation("password").is_ok())
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
        Err(e) => eprintln!("InvalidPasswordError: '{}'", e),
    }
}
