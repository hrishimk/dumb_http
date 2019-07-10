use rand::distributions::Alphanumeric;
use std::collections::HashMap;

use chrono::prelude::*;

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Cookie<'a> {
    name: String,
    value: String,
    http_only: bool,
    secure: bool,
    has_expiry: bool,
    expiry: DateTime<Utc>,
    max_age: u32,
    path: &'a str,
}

impl<'a> Cookie<'a> {
    pub fn with_value(name: String, value: String) -> Self {
        Cookie {
            name,
            value,
            http_only: true,
            secure: false,
            has_expiry: false,
            expiry: Utc::now(),
            max_age: 0,
            path: "/",
        }
    }

    pub fn with_name(name: String) -> Self {
        let value = Self::gen_cookie_val();
        Self::with_value(name, value)
    }

    pub fn set_http_only(&'a mut self, http_only: bool) -> &'a mut Self {
        self.http_only = http_only;
        self
    }

    pub fn set_secure(&'a mut self, http_only: bool) -> &'a mut Self {
        self.secure = http_only;
        self
    }

    pub fn set_max_age(&'a mut self, max_age: u32) -> &'a mut Self {
        self.max_age = max_age;
        self
    }

    fn gen_cookie_val() -> String {
        thread_rng().sample_iter(Alphanumeric).take(30).collect()
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn generate(&self) -> String {
        let mut cookie_string = self.name.to_string();
        //format()
        cookie_string = cookie_string + "=" + self.value.as_str();
        cookie_string = cookie_string + "; Path=" + self.path;
        if self.max_age > 0 {
            cookie_string = cookie_string + "; Max-Age=" + self.max_age.to_string().as_str();
        }
        if self.http_only {
            cookie_string = cookie_string + "; HttpOnly";
        }
        cookie_string
    }
}

#[derive(Debug)]
pub struct CookieManager<T> {
    data: HashMap<String, T>,
}

impl<T> CookieManager<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.data.get_mut(key)
    }

    pub fn set(&mut self, key: &str, value: T) {
        self.data.entry(key.to_string()).or_insert(value);
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}
