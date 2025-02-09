use core::str;
#[allow(unused_imports)]
use http::request::{Builder, Request};
#[allow(unused_imports)]
use std::fmt::Error;
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Sling {
    // http_client
    method: String,
    raw_url: String,
    header: Vec<String>,
    body: Vec<u8>,
    // body_provider interface
    // // response_decoder serde json decoder
}
#[allow(unused)]
impl Sling {
    fn new() -> Self {
        Sling {
            method: String::new(),
            raw_url: String::new(),
            header: Vec::new(),
            body: Vec::new(),
        }
    }
    fn set_uri(&mut self, url: &str) {
        self.raw_url = url.to_string();
    }
    fn raw_uri(&self) -> String {
        self.raw_url.clone()
    }
    fn set_method(&mut self, method: &str) {
        self.method = method.to_string();
    }
    fn method(&self) -> String {
        self.method.clone()
    }
    fn set_header(&mut self, headers: &Vec<String>) {
        if headers.len() < 1 {
            self.header = Vec::new()
        }
        for v in headers {
            self.header.push(v.to_string());
        }
    }
    fn headers(&self) -> Vec<String> {
        self.header.clone()
    }
    fn set_body(&mut self, body_value: Vec<u8>) {
        self.body = body_value
    }
    // TODO:automatically check for headers and if missing
    // provide default headers
    fn build_request(&self) -> Result<http::Request<Vec<u8>>, http::Error> {
        Request::builder()
            .method(self.method.as_str())
            .uri(self.raw_url.as_str())
            // TODO: find alternative to clone bytes
            .body(self.body.clone())
    }
    fn build_request_with_body(
        &mut self,
        body: Vec<u8>,
    ) -> Result<http::Request<Vec<u8>>, http::Error> {
        self.body = body;
        self.build_request()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sling_test() {
        let result = Sling::new();
        assert_eq!(
            result,
            Sling {
                method: "".to_string(),
                raw_url: "".to_string(),
                header: Vec::new(),
                body: Vec::new(),
            }
        );
    }

    #[test]
    fn set_uri() {
        let url_value = String::from("rul");
        let value: &str = url_value.as_str();
        let mut sling = Sling::default();
        sling.set_uri(value);
        assert_eq!(sling.raw_uri(), value)
    }
    #[test]
    fn set_method() {
        let mut sling = Sling::default();
        let value = String::from("GET");
        sling.set_method(value.as_str());
        assert_eq!(sling.method(), value)
    }
    #[test]
    fn set_header() {
        let mut sling = Sling::default();
        let value = vec!["val1".to_string(), "val2".to_string()];
        sling.set_header(&value);
        assert_eq!(sling.headers(), value)
    }
    #[test]
    fn empty_header() {
        let mut sling = Sling::default();
        let value: Vec<String> = Vec::new();
        sling.set_header(&value);
        assert_eq!(sling.headers().len(), 0)
    }
    #[test]
    fn build_request() {
        let mut sling = Sling::default();
        let body_text = "hello";
        let body_bytes = body_text.as_bytes();
        let body_value: Vec<u8> = body_bytes.into();
        sling.set_uri("http://domain.com");
        sling.set_method("GET");
        sling.set_body(body_value);
        let request = sling.build_request().unwrap();
        let result_body = request.into_body();
        assert_eq!(result_body, body_bytes)
    }
    #[test]
    fn request_with_body() {
        let mut sling = Sling::default();
        sling.set_uri("http://domain.com");
        sling.set_method("GET");
        let body_text = "hello".to_string();
        let body_bytes = body_text.as_bytes();
        let body_value: Vec<u8> = body_bytes.into();
        let request = sling.build_request_with_body(body_value).unwrap();
        assert_eq!(request.into_body(), body_bytes)
    }
}
