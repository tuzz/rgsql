use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3003").unwrap();

    for _ in listener.incoming() {

    }
}
