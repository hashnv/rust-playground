struct Number {
    value: u32,
}

impl Number {
    fn new(v: u32) -> Number {
        Number { value: v }
    }

    fn is_prime(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

// impl Iterator for Prime {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         let prime = self.curr;
//         self.curr = self.next;
//         loop {
//             self.next += 1;
//             if self.next().is_prime().is_ok() {
//                 break;
//             }
//         }
//         Some(prime)
//     }
// }

fn main() {
    let test = Number::new(1);
    match test.is_prime() {
        Ok(_) => println!("{} is prime!", test.value),
        Err(_) => println!("{} is not prime!", test.value),
    }
}
