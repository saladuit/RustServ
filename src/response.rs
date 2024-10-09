use std::{
    io::prelude::*, 
    net::TcpStream};
use crate::error::Result;

pub enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        }
    }
}

pub struct StatusLine {
    version: String,
    status_code: StatusCode,
}

impl StatusLine {
    pub fn new(version: &String, status_code: StatusCode) -> StatusLine {
        Self{ version: version.to_string(), status_code}
    }

    pub fn as_str(&self) -> String {
        format!("{} {}", self.version, self.status_code.as_str())
    }
}

pub struct Response {
    pub response: String,
}

impl Response {
    pub fn new(status_line: StatusLine, contents: String) -> Response {
        Response{ response: format!(
            "{}\r\nContent-length: {} \r\n\r\n{}",
            status_line.as_str(),
            contents.len(),
            contents
        )}
    }

    pub fn send(&self, client_stream: &mut TcpStream) -> Result<()> {
        client_stream.write_all(self.response.as_bytes())?;
        Ok(())

    }
}

// #[cfg(tests)]
// mod unit_tests {
//     use super::*;

//     #[test]

// }