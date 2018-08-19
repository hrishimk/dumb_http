use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

//struct HttpRespBuilder {}

pub mod cookie_manager;
pub use cookie_manager::CookieManager;

pub struct HttpResp<'a> {
    stream: &'a mut TcpStream,
    status: u8,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl<'a> HttpResp<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        HttpResp {
            stream: stream,
            status: 200,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn set_body(&mut self, bytes: Vec<u8>) {
        self.set_header("Content-Length".to_string(), bytes.len().to_string());
        self.body = bytes;
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.entry(key).or_insert(value);
    }

    pub fn write(&mut self) {
        let resp = b"HTTP/1.1 200 OK\r\n";
        self.stream.write(resp);

        for (key, value) in &self.headers {
            self.stream.write(key.as_bytes());
            self.stream.write(b": ");
            self.stream.write(value.as_bytes());
            self.stream.write(b"\r\n");
        }

        self.stream.write(b"\r\n");

        self.stream.write(&self.body);

        self.stream.flush();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
