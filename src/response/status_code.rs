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

#[cfg(test)]
mod unit_tests {
    use super::*;
    #[test]
    fn test_status_code_ok() {
        let status = StatusCode::Ok;
        assert_eq!(status.as_str(), "200 OK");
    }

    #[test]
    fn test_status_code_not_found() {
        let status = StatusCode::NotFound;
        assert_eq!(status.as_str(), "404 Not Found");
    }
}
