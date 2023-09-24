fn has_min_length_eight(s: &str) -> Result<(), &'static str> {
    match s.chars().count() >= 8 {
        true => Ok(()),
        false => Err("is fewer than eight characters long"),
    }
}

fn contains_uppercase(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| x.is_uppercase()) {
        true => Ok(()),
        false => Err("does not contain an uppercase character"),
    }
}

fn contains_digit(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| x.is_ascii_digit()) {
        true => Ok(()),
        false => Err("does not contain a digit"),
    }
}

fn contains_punctuation(s: &str) -> Result<(), &'static str> {
    match s.chars().any(|x| !x.is_alphanumeric()) {
        true => Ok(()),
        false => Err("does not contain punctuation"),
    }
}

const PASSWORD_CHECKS: &[fn(&str) -> Result<(), &'static str>] = &[
    has_min_length_eight,
    contains_uppercase,
    contains_digit,
    contains_punctuation,
];

pub fn valid_password(password: &str) -> Result<(), Vec<&str>> {
    let mut results = Vec::new();
    for ele in PASSWORD_CHECKS {
        results.push(ele(password.trim()));
    }
    let errors: Vec<_> = results.into_iter().filter(Result::is_err).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    match errors.is_empty() {
        true => Ok(()),
        false => Err(errors),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_min_length_eight_over() {
        assert!(has_min_length_eight("albatross").is_ok());
    }

    #[test]
    fn test_has_min_length_eight_under() {
        assert!(!has_min_length_eight("monkey").is_ok());
    }

    #[test]
    fn test_has_min_length_eight_exact() {
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
        Err(e) => {
            eprintln!("That password is invalid because it: \n - {}", e.join("\n - "))
        }
    }
}
