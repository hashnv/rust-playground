pub fn square_it(a: i32) -> i64 {
    i64::from(a).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        assert_eq!(square_it(2), 4)
    }

    #[test]
    fn bad_result() {
        assert_ne!(square_it(2), 5)
    }
}

fn main() {
    let num: i32 = 2147483647;
    let num_squared: i64 = square_it(num);
    println!("{} squared is {}.", num, num_squared)
}
