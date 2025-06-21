use std::net::TcpStream;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, stdin};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:3003").unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut buffer = vec![];
    let mut input = stdin().lock();

    while let Ok(num_bytes) = input.read_until(b';', &mut buffer) {
        let connection_closed = num_bytes == 0;
        if connection_closed { break; }

        let message = String::from_utf8_lossy(&buffer);
        writer.write_all(message.trim().as_bytes()).unwrap();
        writer.write_all(b"\0").unwrap();
        writer.flush().unwrap();

        buffer.clear();
        reader.read_until(0, &mut buffer).unwrap();
        let json = String::from_utf8_lossy(&buffer[..buffer.len() - 1]);
        println!("{json}");
        buffer.clear();
    }
}
