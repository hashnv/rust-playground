use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::process::ExitCode;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn count_dupes<T>(vect: Vec<T>) -> u64
where
    T: std::cmp::PartialEq + Clone,
{
    let mut temp: Vec<T> = Vec::new();
    let mut dupe_count: u64 = 0;
    for obj in vect {
        if !temp.contains(&obj) {
            temp.push(obj);
        } else {
            dupe_count += 1;
        }
    }
    dupe_count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> ExitCode {
    let mut program_exit_code = ExitCode::from(0);
    let mut error_count: usize = 0;
    
    let argv: Vec<String> = std::env::args().collect();

    let mut paths: Vec<_> = Vec::new();
    for i in argv[1..].iter() {
        paths.push(Path::new(i));
    }

    for path in paths {
        let mut line_hashes: Vec<u64> = Vec::new();
        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(s) = line {
                    if !s.is_empty() {
                        line_hashes.push(get_hash(s));
                    }   
                }
            }
            eprintln!("[INFO] {} contains {} duplicate(s).", path.display(), count_dupes(line_hashes))
        } else {
            eprintln!("[WARN] Error opening file '{}'", path.display());
            error_count += 1;
        }
    }

    if error_count > 0 {
        program_exit_code = ExitCode::from(1);
    }

    // eprintln!("Finished with {} error(s).", error_count);

    program_exit_code
}
