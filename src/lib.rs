extern crate chrono;
extern crate rand;
extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

use std::fs;

pub mod cookie_manager;
pub use cookie_manager::{Cookie, CookieManager};

#[derive(Debug)]
pub struct HttpResp<'a> {
    stream: &'a mut TcpStream,
    status: u16,
    pub headers: HashMap<String, Vec<String>>,
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
        //self.headers.entry(key).or_insert(value);

        if self.headers.contains_key(&key) {
            self.headers.get_mut(&key).unwrap().push(value);
        } else {
            self.headers.insert(key, vec![value]);
        }
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn redirect(&mut self, page: &str) {
        self.status = 302;
        self.set_header("Location".to_string(), page.to_string());
    }

    pub fn write(&mut self) -> std::io::Result<()> {
        self.stream.write(b"HTTP/1.1 ")?;
        self.stream.write(self.status.to_string().as_bytes())?;
        self.stream.write(b" OK\r\n")?;

        for (key, value) in &self.headers {
            for n in value {
                self.stream.write(key.as_bytes())?;
                self.stream.write(b": ")?;
                self.stream.write(n.as_bytes())?;
                self.stream.write(b"\r\n")?;
            }
        }

        self.stream.write(b"\r\n")?;

        self.stream.write(&self.body)?;

        self.stream.flush()?;

        Ok(())
    }

    pub fn file(&mut self, path: &str) {
        let contents = fs::read(path);
        match contents {
            Ok(x) => {
                self.set_header(
                    "Content-Type".to_string(),
                    Self::get_mime_type(path).to_string(),
                );
                self.set_body(x);
            }
            Err(_x) => {
                self.set_header("Content-Type".to_string(), "text/plain".to_string());
                self.set_body(b"Not Found".to_vec());
            }
        }
    }

    pub fn get_mime_type(path: &str) -> &str {
        match Self::get_ext(path).as_str() {
            ".html" => "text/html",
            ".js" => "text/javascript",
            ".css" => "text/css",
            ".jpeg" => "image/jpeg",
            ".jpg" => "image/jpeg",
            ".png" => "image/png",
            ".svg" => "image/svg+xml",
            _ => "text/html",
        }
    }

    pub fn get_ext(path: &str) -> String {
        let re = Regex::new(r"(\.[0-9a-z]+$)").unwrap();
        match re.captures(path) {
            Some(ext) => return ext[0].to_string(),
            None => return "".to_string(),
        }
    }

    pub fn cookie(&mut self, value: String) {
        self.set_header("Set-Cookie".to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
