use crate::error::Result;

#[derive(PartialEq, Debug)]
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