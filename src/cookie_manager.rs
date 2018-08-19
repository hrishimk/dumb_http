use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct CookieManager {
    data: HashMap<String, String>,
}

impl CookieManager {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str)-> Option<&str>{
        self.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str){
        self.data.entry(key.to_string()).or_insert(value.to_string());
    }

    pub fn delete(&mut self, key: &str){
        self.data.remove(key);
    }

}
