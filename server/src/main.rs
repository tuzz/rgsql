mod evaluator;
mod parser;
mod query_result;

use chumsky::prelude::*;
use chumsky::text::{ident, whitespace};
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use evaluator::*;
use parser::*;
use query_result::*;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3003").unwrap();

    for result in listener.incoming() {
        let stream = result.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let mut buffer = vec![];

        while let Ok(num_bytes) = reader.read_until(0, &mut buffer) {
            let connection_closed = num_bytes == 0;
            if connection_closed { break; }

            let message = String::from_utf8_lossy(&buffer[..buffer.len() - 1]);
            let parse_result = Query::parser().parse(&message);

            let query_result = match parse_result.into_result() {
                Ok(query) => query.evaluate(),
                Err(errors) => QueryResult::from_parse_errors(&errors),
            };

            let json = serde_json::to_string(&query_result).unwrap();
            writer.write_all(json.as_bytes()).unwrap();
            writer.write_all("\0".as_bytes()).unwrap();
            writer.flush().unwrap();
            buffer.clear();
        }
    }
}
