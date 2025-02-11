use std::net::TcpStream;

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
    pub fn connect_with(&self) -> Result<TcpStream, std::io::Error> {
        TcpStream::connect(self.addr)
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
