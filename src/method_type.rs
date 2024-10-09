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

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn get_as_str() {
        assert_eq!(MethodType::Get.as_str(), "GET");
    }
    #[test]
    fn post_as_str() {
        assert_eq!(MethodType::Post.as_str(), "POST");
    }
    #[test]
    fn delete_as_str() {
        assert_eq!(MethodType::Delete.as_str(), "DELETE");
    }
    #[test]
    fn get_from_str() {
        let result = MethodType::from_str("GET");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MethodType::Get);
    }
    #[test]
    fn post_from_str() {
        let result = MethodType::from_str("POST");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MethodType::Post);
    }
    #[test]
    fn delete_from_str() {
        let result = MethodType::from_str("DELETE");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), MethodType::Delete);
    }
    #[test]
    fn invalid_from_str() {
        let result = MethodType::from_str("INVALID");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unkown HTTP method");
    }
}