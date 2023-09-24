fn main() {
    eg1();
    eg2();
}

fn eg1() {
    let cat = String::from("Meow");
    // let cat2 = cat; // breaks rule 2, type "String" has no implicit copy trait.
    let cat2 = cat.clone(); // clone() is implemented instead.
    println!("{}", cat);
    println!("{}", cat2);
}

fn eg2() {
    fn how_long(s: &String) -> usize {
        s.len()
    }

    let cat = String::from("Meow");
    // let length = how_long(cat); // pass by value - doesn't work ! no implicit copy
    let length = how_long(&cat); // pass by reference

    println!("{} is of length {}.", cat, length);
}