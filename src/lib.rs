//! # httpsling
//!
//! `httpsling` provides a test http client
//!
//! sling is an insipration from existing lib written in golang
//! httpsling provides a simple http client for testing .
//! It follows minimal dependency policy using only standard
//! network libraries.
//! # Example
//!```
//! use std::io::BufReader;
//! use std::io::BufRead;
//! use std::io::Write;
//! use std::time::Duration;
//! use httpsling::Sling;
//! let addr = "http://localhost:8888".to_string();
//! let sling = Sling::builder()
//!     .set_read_timeout(Duration::from_millis(10))
//!     .set_uri(addr);
//! let http_client = sling.http_client_used();
//! let mut stream = http_client.connect_to();
//! let mut data = http_client
//!     .build_http_req("GET", "http://localhost:8888/")
//!     .as_bytes()
//!     .to_vec();
//! let bytes_written = stream.write(&mut data).unwrap();
//! println!("bytes written:{:?}", bytes_written);
//! let mut reader = BufReader::new(stream);
//! let received: Vec<u8> = reader
//!     .fill_buf()
//!     .expect("error getting data from stream")
//!     .to_vec();
//! reader.consume(received.len());
//! let data = String::from_utf8(received).expect("invalid utf8 supplied");
//! println!("data received:{data}");
//!
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
use std::{str::FromStr, time::Duration};

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
    raw_url: String,
    read_timeout: Duration,
    body: Vec<u8>,
    header: http::HeaderMap,
}
#[derive(Default, Debug, PartialEq)]
pub struct SlingBuilder {
    http_client: Client,
    raw_url: String,
    read_timeout: Duration,
    body: Vec<u8>,
    header: http::HeaderMap,
}

#[allow(unused)]
impl SlingBuilder {
    /// create a new empty sling similar to using `default`
    fn new() -> Self {
        SlingBuilder {
            raw_url: String::new(),
            read_timeout: Duration::from_millis(10),
            header: HeaderMap::new(),
            body: Vec::new(),
            http_client: Client::default(),
        }
    }
    /// sets the uri base url for server
    pub fn set_uri(mut self, url: String) -> SlingBuilder {
        self.http_client.address(&url);
        self.raw_url = url;
        self
    }

    /// sets the read timeout to be used by underlying client when
    /// communicating with server
    pub fn set_read_timeout(mut self, duration: Duration) -> SlingBuilder {
        self.read_timeout = duration;
        self.http_client.set_read_timeout(duration);
        self
    }

    /// sets the header `http::Header`
    fn set_header(mut self, header_key: &str, value: &str) -> SlingBuilder {
        let header_key_value =
            HeaderName::from_str(header_key).expect("invalid header name provided");
        let header_value = HeaderValue::from_str(value).expect("invalid header value provided");
        self.header.insert(header_key_value, header_value);
        self
    }

    /// body bytes associated with request
    fn set_body(mut self, body_value: Vec<u8>) -> SlingBuilder {
        self.body = body_value;
        self
    }
    fn build(self) -> Sling {
        Sling {
            http_client: self.http_client,
            raw_url: self.raw_url,
            read_timeout: self.read_timeout,
            body: self.body,
            header: self.header,
        }
    }
    pub fn http_client_used(self) -> Client {
        self.http_client
    }
}

#[allow(unused)]
impl Sling {
    /// used to expose builder
    pub fn builder() -> SlingBuilder {
        SlingBuilder::default()
    }
    /// used to get url used for server
    fn raw_uri(&self) -> String {
        self.raw_url.clone()
    }
    /// gets the attached client to slinger
    pub fn http_client(self) -> Client {
        self.http_client
    }
    /// builds a request from supplied path
    pub fn build_request_with_path(&self, method: &str, path: &str) -> String {
        self.http_client.http_req_from_path(method, path)
    }
    /// builds a http request with url supplied
    pub fn build_http_request(&self, method: &str, url: &str) -> String {
        self.http_client.build_http_req(method, url)
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
        let result = SlingBuilder::new().build();
        assert_eq!(
            result,
            Sling {
                read_timeout: Duration::from_millis(10),
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
        let sling = Sling::builder().set_uri(url_value.clone());
        assert_eq!(sling.raw_url, url_value)
    }
    #[test]
    fn set_header() {
        let header_key = "accept";
        let header_value = "application/json";
        let sling = SlingBuilder::default().set_header(&header_key, &header_value);
        assert_eq!(sling.header.len(), 1);
    }
    #[test]
    fn builder() {
        let sling = Sling::default();
        let sling_from_builder = SlingBuilder::new()
            .set_read_timeout(Duration::from_nanos(0))
            .build();
        assert_eq!(sling_from_builder, sling);
    }
    #[test]
    fn connect() {
        let addr = "http://localhost:8888".to_string();
        let sling = SlingBuilder::default()
            .set_read_timeout(Duration::from_millis(10))
            .set_uri(addr);
        let http_client = sling.http_client_used();
        let mut stream = http_client.connect_to();
        let mut data = http_client
            .build_http_req("GET", "http://localhost:8888/")
            .as_bytes()
            .to_vec();
        let bytes_written = stream.write(&mut data).unwrap();
        println!("bytes written:{:?}", bytes_written);
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
