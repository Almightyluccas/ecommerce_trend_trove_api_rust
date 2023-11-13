use std::net::TcpStream ;
use std::io::{ Read, Write };
use postgres::Client;

use crate::models::response;
use crate::http::users_handlers as users;

pub fn handle_client(mut stream: TcpStream, mut client: &Option<Client>) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match client {
                Some(client) => {
                    match &*request {
                        r if r.starts_with("POST /users/register") => users::post::handle_register_request(r, &client),
                        r if r.starts_with("GET /users/") => users::get::handle_get_request(r, &client),
                        r if r.starts_with("GET /users") => users::get::handle_get_all_request(r, &client),
                        r if r.starts_with("PUT /users/") => users::put::handle_put_request(r, &client),
                        r if r.starts_with("DELETE /users/") => users::delete::handle_delete_request(r, &client),
                        _ => (response::NOT_FOUND.to_string(), " 404 Not Found".to_string()),
                    }
                }
                None => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),   
            };
                
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => println!("Error: {}", e),
    }
}
