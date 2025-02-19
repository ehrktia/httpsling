//! # httpsling
//!
//! `httpsling` provides a test http client
//!
//! sling is an insipration from existing lib written in golang
//! httpsling provides a simple http client for testing .
//! It follows minimal dependency policy using only standard
//! network libraries.
//! # Example
//! ```
//!use httpsling::Sling;
//!use std::io::BufRead;
//!use std::io::Write;
//!use std::io::BufReader;
//!use std::time::Duration;
//! let addr = "http://localhost:8888".to_string();
//! let sling = Sling::default();
//! let mut http_client = sling.http_client();
//! http_client.address(&addr);
//! let mut stream = http_client.connect_to();
//! let mut data = http_client
//!     .build_http_req("GET", "http://localhost:8888/")
//!     .as_bytes()
//!     .to_vec();
//! let bytes_written = stream.write(&mut data).unwrap();
//! println!("bytes written:{:?}", bytes_written);
//! stream
//!     .set_read_timeout(Some(Duration::from_millis(10)))
//!     .expect("error setup read timeout");
//! let mut reader = BufReader::new(stream);
//! let received: Vec<u8> = reader
//!     .fill_buf()
//!     .expect("error getting data from stream")
//!     .to_vec();
//! reader.consume(received.len());
//! let data = String::from_utf8(received).expect("invalid utf8 supplied");
//! println!("data received:{data}");
//! ```
//!
//! This is a blocking/synchronus client, plans in future to make this async.
//! Focus of this client is to help with testing http servers and apis.
//! The client can be used to connect with http server, fetch
//! response code and response body.

use client::client::Client;
use core::str;
#[allow(unused_imports)]
use http::request::{Builder, Request};
use http::{HeaderMap, HeaderName, HeaderValue};
#[allow(unused_imports)]
use std::fmt::Error;
use std::str::FromStr;

pub mod client;

/// Sling is used to interact with http server.
/// it holds a `http_client` which is used to interact
/// with http server.
/// `raw_url` which can point to server which you prefer to connect
/// with
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Sling {
    http_client: Client,
    method: String,
    raw_url: String,
    body: Vec<u8>,
    header: http::HeaderMap,
}

#[allow(unused)]
impl Sling {
    /// create a new empty sling similar to using `default`
    fn new() -> Self {
        Sling {
            method: String::new(),
            raw_url: String::new(),
            header: HeaderMap::new(),
            body: Vec::new(),
            http_client: Client::default(),
        }
    }

    /// sets the uri
    fn set_uri(&mut self, url: &str) {
        self.raw_url = url.to_string();
    }

    /// used to get url used for server
    fn raw_uri(&self) -> String {
        self.raw_url.clone()
    }

    /// sets method for request
    fn set_method(&mut self, method: &str) {
        self.method = method.to_string();
    }

    /// provides method used
    fn method(&self) -> String {
        self.method.clone()
    }

    /// sets the header `http::Header`
    fn set_header(&mut self, header_key: &str, value: &str) -> bool {
        let header_key_value =
            HeaderName::from_str(header_key).expect("invalid header name provided");
        let header_value = HeaderValue::from_str(value).expect("invalid header value provided");
        self.header.insert(header_key_value, header_value).is_none()
    }

    /// body bytes associated with request
    fn set_body(&mut self, body_value: Vec<u8>) {
        self.body = body_value
    }

    /// builds a http request
    fn build_request(&mut self) -> Result<http::Request<Vec<u8>>, http::Error> {
        if self.header.is_empty() {
            return Request::builder()
                .method(self.method.as_str())
                .uri(self.raw_url.as_str())
                .header("Accept", "application/json")
                .body(self.body.clone());
        } else {
            let mut request = Request::builder()
                .method(self.method.as_str())
                .uri(self.raw_url.as_str())
                .header("Accept", "application/json")
                .body(self.body.clone())?;
            for (k, v) in self.header.iter() {
                let val = v.to_str().expect("invalid header value received");
                request.headers_mut().insert(k, v.clone()).unwrap();
            }
            return Ok(request);
        }
    }
    /// builds request with a body
    fn build_request_with_body(
        &mut self,
        body: Vec<u8>,
    ) -> Result<http::Request<Vec<u8>>, http::Error> {
        self.body = body;
        self.build_request()
    }
    /// build a client with `base_url` for server
    fn client_with_base_url(&mut self, url: String) {
        self.http_client.address(&url);
    }
    /// gets configured default http client
    pub fn http_client(&self) -> Client {
        self.http_client.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;
    #[allow(unused_imports)]
    use std::io::BufReader;
    #[allow(unused_imports)]
    use std::{io, time::Duration};
    #[allow(unused_imports)]
    use std::{
        io::{BufWriter, Read, Write},
        net::Shutdown,
    };

    use http::Uri;

    use super::*;
    #[test]
    fn sling_test() {
        let result = Sling::new();
        assert_eq!(
            result,
            Sling {
                method: "".to_string(),
                raw_url: "".to_string(),
                header: HeaderMap::with_capacity(0),
                body: Vec::new(),
                http_client: Client::default(),
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
        let header_key = "accept";
        let header_value = "application/json";
        let result = sling.set_header(&header_key, &header_value);
        assert_eq!(sling.header.len(), 1);
        assert_eq!(result, true)
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
    #[test]
    fn connect() {
        let addr = "http://localhost:8888".to_string();
        let sling = Sling::default();
        let mut http_client = sling.http_client();
        http_client.address(&addr);
        let mut stream = http_client.connect_to();
        let mut data = http_client
            .build_http_req("GET", "http://localhost:8888/")
            .as_bytes()
            .to_vec();
        let bytes_written = stream.write(&mut data).unwrap();
        println!("bytes written:{:?}", bytes_written);
        stream
            .set_read_timeout(Some(Duration::from_millis(10)))
            .expect("error setup read timeout");
        let mut reader = BufReader::new(stream);
        let received: Vec<u8> = reader
            .fill_buf()
            .expect("error getting data from stream")
            .to_vec();
        reader.consume(received.len());
        let data = String::from_utf8(received).expect("invalid utf8 supplied");
        println!("data received:{data}");
    }
    #[test]
    fn url() {
        let url: Uri = "localhost:8888".parse().unwrap();
        let host = url.host().expect("invalid host provided");
        println!("uri:{:?}", host);
        assert_eq!(host, "localhost");
        let port = url.port().expect("invalid port supplied");
        println!("port:{:?}", port.as_str());
        assert_eq!(port.as_str(), "8888");
    }
}
