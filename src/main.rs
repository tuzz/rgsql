mod parser;
mod query_result;

use chumsky::prelude::*;
use chumsky::text::{ident, whitespace};
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use parser::*;
use query_result::*;
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
            let message = String::from_utf8_lossy(&buffer[..buffer.len() - 1]);
            let parse_result = Query::parser().parse(&message);

            let query_result = match parse_result.into_result() {
                Ok(query) => {
                    let rows = vec![query.select.select_list.literals];
                    let column_names = query.select.select_list.aliases;
                    QueryResult::Ok(SuccessResult { rows, column_names })
                },
                Err(errors) => {
                    let error_type = "parsing_error".to_string();
                    let error_message = errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
                    QueryResult::Error(ErrorResult { error_type, error_message })
                },
            };

            let json = serde_json::to_string(&query_result).unwrap();
            writer.write_all(json.as_bytes()).unwrap();
            writer.write_all("\0".as_bytes()).unwrap();
            buffer.clear();
        }
    }
}
