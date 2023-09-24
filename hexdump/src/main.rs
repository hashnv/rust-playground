use std::fs::File;
use std::io::{self, BufReader, Read};

const BUFFERSIZE: usize = 16;

type HexDumpBuffer = [u8; BUFFERSIZE];
const EMPTYBUFFER: HexDumpBuffer = [0; BUFFERSIZE];

fn replace_whitespace(s: &str) -> String {
    s.clone()
        .chars()
        .map(|c| match c.is_ascii_graphic() || c == ' ' {
            true => c,
            false => '.',
        })
        .collect()
}

fn main() -> io::Result<()> {
    let file = File::open("foo.txt")?;

    let mut reader = BufReader::new(file);
    let mut buffer: HexDumpBuffer = EMPTYBUFFER;
    let mut position: usize = 0;
    let mut utf8_decoded;
    let mut no_whitespace;

    let mut bytes_read = reader.read(&mut buffer)?;

    while bytes_read > 0 {
        utf8_decoded = String::from_utf8_lossy(&buffer);
        no_whitespace = replace_whitespace(&utf8_decoded);
        
        println!("{:08x} {:02x?} {}", position, buffer, no_whitespace);

        position += BUFFERSIZE;
        buffer = EMPTYBUFFER; // zero out buffer
        bytes_read = reader.read(&mut buffer)?;
    }
    Ok(())
}
