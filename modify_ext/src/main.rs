fn modify_ext(old_file:&mut String) -> usize {
    old_file.push_str(".json");
    old_file.len()
}

fn main() {
    let mut tempfile: String = String::from("myfile");
    let mut namelen: usize = tempfile.len();

    println!("filename is {} chars long", namelen);
    
    namelen = modify_ext(&mut tempfile);
    println!("filename is now: {}", tempfile);
    println!("filename is now {} chars long", namelen);
}
