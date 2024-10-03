use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7879")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
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
            "DELETE" => Ok(MethodType::Delete),
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

pub struct Request {
    request_line: RequestLine,
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

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = Request::build(buf_reader)?;
    let http_response = Response::build(
        http_request.request_line.version, http_request.request_line.request_target)?;
    http_response.send(&mut stream)?;
    Ok(())
}