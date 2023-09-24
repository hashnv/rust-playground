use std::process;

pub fn sanitize(s: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    s.to_lowercase()
        .chars()
        .filter(|c| alphabet.contains(*c))
        .collect()
}

pub fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let (s1, s2) = (sanitize(s1), sanitize(s2));
    sort_string(&s1) == sort_string(&s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram_easy() {
        assert!(is_anagram("anger", "range"))
    }

    #[test]
    fn test_is_anagram_medium() {
        assert!(is_anagram("conversation", "voices rant on"))
    }

    #[test]
    fn test_is_anagram_hard() {
        assert!(is_anagram("snooze alarms", "alas, no more Z's"))
    }

    #[test]
    fn test_is_anagram_failure() {
        assert!(!is_anagram("anagram", "soup"))
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 3 {
        eprintln!("Not enough args supplied!");
        process::exit(1);
    }

    let (s1, s2) = (argv[1].as_str(), argv[2].as_str());

    match is_anagram(s1, s2) {
        true => println!("'{}' is an anagram of '{}'.", s1, s2),
        _ => println!("'{}' is NOT an anagram of '{}'.", s1, s2),
    }
}
