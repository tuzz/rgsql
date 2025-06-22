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
                Ok(Query::Select(select)) => {
                    let rows = vec![select.select_list.literals];
                    let column_names = select.select_list.aliases;
                    QueryResult::Ok(SuccessResult { rows, column_names })
                },
                Ok(Query::CreateTable(_create_table)) => {
                    QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
                },
                Ok(Query::DropTable(_drop_table)) => {
                    QueryResult::Ok(SuccessResult { rows: vec![], column_names: vec![] })
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
            writer.flush().unwrap();
            buffer.clear();
        }
    }
}
