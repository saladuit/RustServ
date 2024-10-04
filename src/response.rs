use std::{
    fs,
    io::prelude::*, 
    net::{TcpStream}};
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
    status_line: StatusLine,
    contents: String,
}

impl Response {
    pub fn new(status_line: StatusLine, contents: String) -> Response {
        Response{ status_line, contents}
    }

    pub fn build(version: String, request_target: String) -> Result<Response> {
        let contents = fs::read_to_string(format!(".{}", &request_target));
        match contents {
            Ok(contents) => Ok(Response::new(StatusLine::new(&version, StatusCode::Ok), contents)),
            Err(_) => Ok(Response::new(StatusLine::new(&version, StatusCode::NotFound), String::new())),
        }
    }

    pub fn send(&self, client_stream: &mut TcpStream) -> Result<()> {
        let length = self.contents.len();
        let response = format!(
            "{}\r\nContent-length: {} \r\n\r\n{}",
            self.status_line.as_str(),
            length,
            self.contents
        );

        client_stream.write_all(response.as_bytes())?;
        Ok(())

    }
}