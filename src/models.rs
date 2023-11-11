
use serde_derive::{ Serialize, Deserialize };
use std::env;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

pub mod response {
    pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
    pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    pub const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
}

pub const DB_URL: &str = env!("DATABASE_URL");



    

