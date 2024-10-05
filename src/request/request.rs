
use crate::error::Result;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream};

use crate::request::request_line::RequestLine;

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Vec<String>
}

impl Request {
    pub fn build(buf_reader: BufReader<&mut TcpStream>) -> Result<Request> {
        let mut lines = buf_reader.lines();
        let request_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(Box::new(e)),
            None => return Err("Request line not found".into()),
        };
        let request_line = RequestLine::build(request_line)?;

        let headers: Vec<String> = lines
        .take_while(|result| 
            match result {
            Ok(line) => !line.is_empty(),
            Err(_) => false,
        })
        .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;

        Ok(Self {request_line, headers})
    }
}