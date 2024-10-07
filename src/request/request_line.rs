use crate::method_type::MethodType;
use crate::error::Result;

#[derive(Debug)]
pub struct RequestLine {
    pub method: MethodType,
    pub request_target: String,
    pub version: String,
}

impl RequestLine {
    pub fn build(request_line: String) -> Result<RequestLine> {
        let parts: Vec<&str> = request_line.split_ascii_whitespace().collect();

        if parts.len() < 1 {
            return Err(format!("Too little HTTP request line arguments: {}", request_line).into());
        }
        if parts.len() > 3 {
            return Err(format!("Too many HTTP request line arguments: {}", request_line).into());
        }
        
        let method = MethodType::from_str(parts[0])?;
        
        
        let request_target = parts[1].to_string();
        if parts.len() == 1 {
            return Err(format!("No request target in HTTP request line argument").into());
        }
        if parts.len() == 2 {
            return Err(format!("HTTP version missing in HTTP request line argument missing").into());
        }
        let version = parts[2].to_string();
        if !version.starts_with("HTTP/") {
            return Err(format!("Invalid HTTP version formatting: {}", version).into());
        }
        Ok(Self {method, request_target, version})
    }
}

#[cfg(test)]
mod unit_tests {

    use super::*;
    
    fn assert_valid_request_line(method: &str, request_target: &str, version: &str) {
        let request_line = format!("{} {} {}", method, request_target, version);
        let result = RequestLine::build(request_line);
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.method, MethodType::from_str(method).unwrap());
        assert_eq!(req.request_target, "/index.html");
        assert_eq!(req.version, "HTTP/1.1");

    }
    #[test]
    fn get_request_line() {
        assert_valid_request_line("GET", "/index.html", "HTTP/1.1");
    }
    #[test]
    fn empty_request_target() {
        let method = "GET";
        let request_target = " ";
        let version = "";
        let request_line = format!("{} {} {}", method, request_target, version);
        let result = RequestLine::build(request_line);
        assert!(result.is_err());
        // assert_eq!()

    }
    #[test]
    fn delete_request_line() {
        assert_valid_request_line("DELETE", "/index.html", "HTTP/1.1");
    }
    #[test]
    fn post_request_line() {
        assert_valid_request_line("POST", "/index.html", "HTTP/1.1");
    }
    #[test]
    fn short_invalid_request_target() {
        let request_line = "GET /index.html".to_string();
        let result = RequestLine::build(request_line);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(),
    "Invalid request line format: GET /index.html");
    }
    #[test]
    fn long_invalid_request_target() {
        let request_line = "GET /index.html HTTP/1.1 Extra".to_string();
        let result = RequestLine::build(request_line);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(),
    "Too many HTTP request line arguments: GET /index.html HTTP/1.1 Extra");
    }
    
    
}