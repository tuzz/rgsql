use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3003").unwrap();

    for result in listener.incoming() {
        let stream = result.unwrap();
        let mut writer = stream.try_clone().unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(&stream);

        while reader.read_until(0, &mut buffer).is_ok() {
            writer.write_all("\0".as_bytes()).unwrap();
        }
    }
}
