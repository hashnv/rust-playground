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
    let file = File::open("random.bytes")?;
    let mut reader = BufReader::new(file);
    let mut buffer: HexDumpBuffer;
    let mut position: usize = 0;
    loop {
        buffer = EMPTYBUFFER; // zero out buffer
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let utf8_decoded = String::from_utf8_lossy(&buffer);
        let no_whitespace = replace_whitespace(&utf8_decoded);
        println!("{:08x} {:02x?} {}", position, buffer, no_whitespace);
        position += BUFFERSIZE;
    }
    Ok(())
}
