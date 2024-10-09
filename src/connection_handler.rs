use std::fs;
use std::{
    net::TcpStream,
    io::BufReader,
};

use crate::error::Result;
use crate::request::Request;
use crate::response::{StatusLine, Response, StatusCode};


pub struct ConnectionHandler<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    pub fn handle_connection(&mut self) -> Result<()> {
        let buf_reader = BufReader::new(&mut *self.stream);
        let http_request = Request::build(buf_reader)?;
        let contents = fs::read_to_string(http_request.request_line.request_target);
        let http_response  = match contents {
            Ok(contents) => Response::new(StatusLine::new(&http_request.request_line.version, StatusCode::Ok), contents),
            Err(_) => Response::new(StatusLine::new(&http_request.request_line.version, StatusCode::NotFound), String::new()),
        };
        http_response.send(&mut self.stream)?;
        Ok(())
    }
}
