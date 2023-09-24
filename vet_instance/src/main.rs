use std::fmt;

// uncomment to automatically add Debug trait
// #[derive(Debug)]
struct Pet {
    name: String,
    age: u8,
    last_bill: i32,
}

impl Pet {
    fn new(name: &str, age: u8, last_bill: i32) -> Pet {
        Pet {
            name: String::from(name),
            age: age,
            last_bill: last_bill,
        }
    }
}

// customized debug trait
impl fmt::Debug for Pet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, age {}, ${}", self.name, self.age, self.last_bill)
    }
}

fn main() {
    let pet1 = Pet::new("bob", 18, 123);
    println!("{:?}", pet1)
}
