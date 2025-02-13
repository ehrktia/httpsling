#[allow(unused_imports)]
use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

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
    // TODO: find a way to write test and skip in ci and local using env var
    // setup
    pub fn connect_stream(&self) -> TcpStream {
        TcpStream::connect(self.addr).expect("error connecting to server to write")
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
}
