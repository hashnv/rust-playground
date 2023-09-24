fn modify_ext(old_file:&mut String) {
    old_file.push_str(".json");
}

fn main() {
    let mut tempfile: String = String::from("myfile");

    println!("filename is {} chars long", tempfile.len());
    
    modify_ext(&mut tempfile);

    println!("filename is now: {}", tempfile);
    println!("filename is now {} chars long", tempfile.len());
}
