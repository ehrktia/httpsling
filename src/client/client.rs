#[allow(unused_imports)]
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use http::Uri;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn address(&mut self, add: &String) {
        self.base_url = add.clone();
    }

    pub fn connect_to(&self) -> TcpStream {
        let mut url = self.base_url.clone();
        if url.starts_with("http://") {
            url = url.replace("http://", "");
        }
        TcpStream::connect(url).expect("error connecting to server to write")
    }

    fn check_default_path(&self) -> bool {
        let base_len = self.base_url.len();
        let (_first, last) = self.base_url.split_at(base_len - 1);
        return last == "/";
    }

    pub fn http_req_from_path(&self, method: &str, path: &str) -> String {
        let mut base = self.base_url.to_string();
        if path.starts_with("/") {
            if self.check_default_path() {
                return self.base_url.clone();
            }
            base.push_str(path);
            return self.build_http_req(method, &base);
        }
        path.to_string().insert_str(0, "/");
        base.push_str(path);
        self.build_http_req(method, &base)
    }

    fn get_path(&self, url: &Uri) -> Result<String, &'static str> {
        if url.path() == "" {
            let e = Err("invalid path supplied");
            return e;
        }
        match url.path_and_query() {
            Some(pq) => return Ok(pq.to_string()),
            _ => return Err("invalid query param supplied"),
        }
    }

    pub fn build_http_req(&self, method: &str, url: &str) -> String {
        let uri: Uri = url.parse().expect("invalid url supplied");
        let path_query = self
            .get_path(&uri)
            .expect("invalid query param and path supplied");
        let hostname = uri.host().expect("invalid host provided");
        let port = uri.port().expect("invalid port supplied");
        let host_name = [hostname, port.as_str()].join(":");
        let mut req = String::new();
        req.push_str(method.to_uppercase().as_str());
        req.push_str(" ");
        req.push_str(path_query.as_str());
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
        let test_addr = String::from("127.0.0.1:8888");
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
    #[test]
    fn build_req_from_path() {
        let mut client = Client::default();
        let addr = String::from("http://localhost:8080");
        client.address(&addr);
        let result = client.http_req_from_path("GET", "/");
        let want = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nAccept: */*\r\n\r\n";
        assert_eq!(result, want)
    }
    #[test]
    fn url_with_no_slash() {
        let addr = String::from("home");
        let mut client = Client::default();
        client.address(&addr);
        let url: Uri = addr.parse().unwrap();
        let v = match client.get_path(&url) {
            Ok(v) => v,
            Err(e) => e.to_string(),
        };
        assert_ne!(v, addr);
    }

    #[test]
    fn connect_stream() {
        let ci = option_env!("CI").is_some();
        if !ci {
            let addr = String::from("http://localhost:8888");
            let mut client = Client::default();
            client.address(&addr);
            client.connect_to();
        } else {
            println!("running in ci");
        }
    }
    #[test]
    fn check_default_path() {
        let mut client = Client::default();
        let addr = String::from("http://localhost:8888/");
        client.address(&addr);
        assert_eq!(client.check_default_path(), true)
    }
    #[test]
    fn check_default_path_false() {
        let mut client = Client::default();
        let addr = String::from("http://localhost:8888");
        client.address(&addr);
        assert_eq!(client.check_default_path(), false);
    }
}
