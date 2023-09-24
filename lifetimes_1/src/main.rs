// fn get_shortest(s1: &str, s2: &str) -> &str {
fn get_shortest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    match s1.chars().count() < s2.chars().count() {
        true => s1,
        false => s2,
    }
}

fn main() {
    let a = "tiny";
    let b = "massive";
    println!("{}", get_shortest(a, b));
}
