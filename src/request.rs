use crate::method_type::MethodType;
use crate::error::Result;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream};

pub struct RequestLine {
    pub method: MethodType,
    pub request_target: String,
    pub version: String,
}

impl RequestLine {
    pub fn build(request_line: String) -> Result<RequestLine> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(format!("Invalid request line format: {}", request_line).into());
        }
        
        let method = MethodType::from_str(parts[0])?;
        
        let request_target = parts[1].to_string();
        if request_target.is_empty() {
            return Err(format!("Invalid request target: {}", request_target).into());
        }
        let version = parts[2].to_string();
        if !version.starts_with("HTTP/") {
            return Err(format!("Invalid HTTP version: {}", version).into());
        }
        Ok(Self {method, request_target, version})
    }
}

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