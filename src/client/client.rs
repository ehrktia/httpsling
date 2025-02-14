#[allow(unused_imports)]
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use http::Uri;

#[derive(Default, Debug, PartialEq, Clone, Eq)]
pub struct Client<'a> {
    addr: &'a str,
}

impl<'a> Client<'a> {
    pub fn address(&mut self, add: &'a str) -> Self {
        let mut client = self.clone();
        client.addr = add;
        client
    }

    pub fn get_address(&self) -> String {
        self.addr.to_string()
    }

    pub fn connect_stream(&self) -> TcpStream {
        TcpStream::connect(self.addr).expect("error connecting to server to write")
    }

    pub fn build_http_req(&self, method: &str, url: &str) -> String {
        let uri: Uri = url.parse().expect("invalid url supplied");
        let hostname = uri.host().expect("invalid host provided");
        let port = uri
            .port()
            .expect("invalid port supplied")
            .as_str()
            .to_owned();
        let host_name = hostname.to_string() + ":" + port.as_str();
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
        let client = Client::default().address(&test_addr);
        assert_eq!(client.addr, test_addr);
        assert_eq!(client.addr, client.get_address())
    }

    #[test]
    fn build_request() {
        let client = Client::default();
        let result = client.build_http_req("GET", "http://localhost:8888/");
        let want = "GET / HTTP/1.1\r\nHost: localhost:8888\r\nAccept: */*\r\n\r\n";
        assert_eq!(result, want.to_string());
    }
}
