use std::fs::read_to_string;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut exit_code = ExitCode::from(0);
    let argv: Vec<String> = std::env::args().collect();

    let mut paths: Vec<_> = Vec::new();
    for i in argv[1..].iter() {
        paths.push(Path::new(i));
    }

    let mut error_count: usize = 0;
    for i in paths {
        let contents = read_to_string(i);

        if contents.is_ok() {
            print!("{}", contents.unwrap())
        } else {
            error_count += 1;
        }
    }
 
    if error_count > 0 {
        exit_code = ExitCode::from(1);
    }

    eprintln!("Finished with {} error(s).", error_count);

    ExitCode::from(exit_code)
}
