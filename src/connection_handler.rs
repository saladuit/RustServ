use std::{
    net::TcpStream,
    io::BufReader,
};

use crate::error::Result;
use crate::request::Request;
use crate::response;

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
        let http_response = response::Response::build(
            http_request.request_line.version, http_request.request_line.request_target)?;
            http_response.send(&mut self.stream)?;
        Ok(())
    }
}
