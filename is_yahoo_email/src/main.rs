fn is_yahoo_email(email:&String) -> bool {
    let domain = "@yahoo.com";
    email.ends_with(domain)
}

fn main() {
    let email = String::from("example@yahoo.com");

    if is_yahoo_email(&email) {
        println!("{} is a Yahoo account", email);
    } else {
        println!("{} is not a Yahoo account", email);
    }
}