use std::fs::File;
use std::path::Path;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();

    if argc < 2 {
        panic!("Missing arg");
    }

    let path = Path::new(&argv[1]);
    
    // let f = match File::open(&path) {
    match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("Could not open {}: {}", path.display(), e),
    };

    println!("Loaded file: '{}'", path.display());
}
