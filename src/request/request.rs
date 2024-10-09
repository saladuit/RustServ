use std::io::{prelude::*, BufReader};
    
use crate::error::Result;
use crate::request::request_line::RequestLine;

pub struct Request {
    pub request_line: RequestLine,
    pub headers: Vec<String>
}

impl Request {
    pub fn build<R: Read>(buf_reader: BufReader<R>) -> Result<Request> {
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

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::{collections::VecDeque, io::Cursor};

    fn get_mock_request(request: Vec<String>) -> BufReader<Cursor<Vec<u8>>> {
        let request_data = VecDeque::from(request)
        .into_iter()
        .flat_map(|s| s.into_bytes())
        .collect();
        let cursor = Cursor::new(request_data);
        BufReader::new(cursor)
    }

    #[test]
    fn test_request_build_success() {
        let buf_reader = get_mock_request(
            vec!["GET / HTTP/1.1\r\n".to_string(),
            "Host: example.com\r\n".to_string(),
             "\r\n".to_string()]);
        let request = Request::build(buf_reader);
        assert!(request.is_ok());
        let request = request.unwrap();
        assert_eq!(request.request_line.method.as_str(), "GET");
        assert_eq!(request.request_line.request_target, "/");
        assert_eq!(request.request_line.version, "HTTP/1.1");
        assert_eq!(request.headers, vec!["Host: example.com"]);
    }

    // #[test]
    // fn test_request_build_missing_request_line() {
    //     let request_data = "";
    //     let cursor = Cursor::new(request_data);
    //     let buf_reader = BufReader::new(cursor);

    //     let result = Request::build(buf_reader);

    //     assert!(result.is_err());
    // }

    // #[test]
    // fn test_request_build_with_headers() {
    //     let request_data = "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
    //     let cursor = Cursor::new(request_data);
    //     let buf_reader = BufReader::new(cursor);

    //     let request = Request::build(buf_reader).unwrap();

    //     assert_eq!(request.request_line.method, "GET");
    //     assert_eq!(request.request_line.uri, "/");
    //     assert_eq!(request.request_line.version, "HTTP/1.1");
    //     assert_eq!(request.headers, vec!["Host: example.com", "User-Agent: test"]);
    // }
}