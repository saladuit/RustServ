use crate::response::StatusCode;

pub struct StatusLine {
    version: String,
    status_code: StatusCode,
}

impl StatusLine {
    pub fn new(version: &String, status_code: StatusCode) -> StatusLine {
        Self {
            version: version.to_string(),
            status_code,
        }
    }

    pub fn as_str(&self) -> String {
        format!("{} {}", self.version, self.status_code.as_str())
    }
}
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_status_line_new() {
        let version = "HTTP/1.1".to_string();
        let status_line = StatusLine::new(&version, StatusCode::Ok);

        assert_eq!(status_line.version, version);
        assert_eq!(status_line.status_code, StatusCode::Ok);
    }

    #[test]
    fn test_status_line_as_str() {
        let version = "HTTP/1.1".to_string();
        let status_code = StatusCode::Ok;
        let status_line = StatusLine::new(&version, status_code);

        assert_eq!(status_line.as_str(), "HTTP/1.1 200 OK");
    }
}
