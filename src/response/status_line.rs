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
