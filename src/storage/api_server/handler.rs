// src/storage/api_server/handler.rs

pub trait Handler {
    fn handle(&self, request: &str) -> String;
}
