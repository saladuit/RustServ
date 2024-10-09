use crate::response::{StatusCode, StatusLine};

#[derive(Debug)]
pub struct Response {
    response: String,
}

impl Response {
    pub fn new(status_line: StatusLine, contents: String) -> Response {
        Response {
            response: format!(
                "{}\r\nContent-length: {}\r\n\r\n{}",
                status_line.as_str(),
                contents.len(),
                contents
            ),
        }
    }
    pub fn get_response(&self) -> &[u8] {
        self.response.as_bytes()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn ok_response() {
        let response = Response::new(
            StatusLine::new(&"HTTP/1.1".to_string(), StatusCode::Ok),
            "Hello World!".to_string(),
        );
        assert_eq!(
            response.get_response(),
            "HTTP/1.1 200 OK\r\nContent-length: 12\r\n\r\nHello World!".as_bytes());
    }
    #[test]
    fn not_found_reponse() {
        let response = Response::new(
            StatusLine::new(&"HTTP/1.1".to_string(), StatusCode::NotFound),
            "".to_string(),
        );
        assert_eq!(
            response.get_response(),
            "HTTP/1.1 404 Not Found\r\nContent-length: 0\r\n\r\n".as_bytes());
    }
}
