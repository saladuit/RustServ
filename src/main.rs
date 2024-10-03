use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;


fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let http_request = handle_connection(stream)?;
                println!("Request: {:#?}", http_request);
            }
            Err(e) => {
                println!("Connection failed: {e}");
            }
        }
    }
    Ok(())
}
pub enum MethodType {
    Get,
    Post,
    Delete,
}

impl MethodType {
    pub fn as_str(&self) -> &str {
        match self {
            MethodType::Get => "GET",
            MethodType::Post => "POST",
            MethodType::Delete => "DELETE",
        }
    }
    pub fn from_str(method: &str) -> Result<MethodType> {
        match method {
            "GET" => Ok(MethodType::Get),
            "POST"=> Ok(MethodType::Post),
            "Delete" => Ok(MethodType::Delete),
            _ => Err("Unkown HTTP method".into())
        }
    }
}
pub struct RequestLine {
    method: MethodType,
    request_target: String,
    version: String,
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

struct Response {
    response_line: RequestLine,
    contents: String,
    bytes_sent: usize,
}

impl Response {
    pub fn build(path: String) {

    }
    pub fn send(client_fd: i32) {

    }
}

pub struct Request {
    request_line: String,
    headers: Vec<String>
}

impl Request {
    pub fn build(buf_reader: BufReader<&mut TcpStream>) -> Result<Request> {
        let mut lines = buf_reader.lines();
        let request_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(Box::new(e)),
            None => return Err("Request line not found".into()),
        };

        let headers: Vec<String> = lines
        .take_while(|result| match result {
            Ok(line) => !line.is_empty(),
            Err(_) => false,
        })
        .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;

        Ok(Self {request_line, headers})
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<Vec<String>> {
    let buf_reader = BufReader::new(&mut stream);
    
    let http_request = buf_reader
    .lines()
    .take_while(|result| match result {
        Ok(line) => !line.is_empty(),
        Err(_) => false
    })
    .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html")?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;
    Ok(http_request)
}