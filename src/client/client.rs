#[allow(unused_imports)]
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use http::Uri;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Client<'a> {
    base_url: &'a str,
}

impl<'a> Client<'a> {
    pub fn address(&mut self, add: &'a str) {
        self.base_url = add;
    }

    pub fn connect_stream(&self) -> TcpStream {
        TcpStream::connect(self.base_url).expect("error connecting to server to write")
    }

    pub fn build_http_req(&self, method: &str, url: &str) -> String {
        let uri: Uri = url.parse().expect("invalid url supplied");
        let hostname = uri.host().expect("invalid host provided");
        let port = uri.port().expect("invalid port supplied");
        let host_name = [hostname, port.as_str()].join(":");
        let mut path = uri.path();
        if path == "" {
            path = "/"
        }
        let mut req = String::new();
        req.push_str(method.to_uppercase().as_str());
        req.push_str(" ");
        req.push_str(path);
        req.push_str(" ");
        req.push_str("HTTP/1.1\r\n");
        req.push_str("Host: ");
        req.push_str(&host_name);
        req.push_str("\r\n");
        req.push_str("Accept: ");
        req.push_str("*/*\r\n\r\n");
        req
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn address() {
        let test_addr = "127.0.0.1:8888";
        let mut client = Client::default();
        client.address(&test_addr);
        assert_eq!(client.base_url, test_addr)
    }

    #[test]
    fn build_request() {
        let client = Client::default();
        let result = client.build_http_req("GET", "http://localhost:8888/");
        let want = "GET / HTTP/1.1\r\nHost: localhost:8888\r\nAccept: */*\r\n\r\n";
        assert_eq!(result, want.to_string());
    }
}
