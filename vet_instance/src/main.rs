use std::fmt;

struct Pet {
    name: String,
    age: u8,
    last_bill: i32,
}

impl fmt::Debug for Pet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, age {}, ${}", self.name, self.age, self.last_bill)
    }
}

fn main() {
    let pet1 = Pet {
        name: String::from("Mr pet"),
        age: 8,
        last_bill: 8012,
    };

    println!("{:?}", pet1)
}
