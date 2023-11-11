use database::errors::InitializaitonError;
use postgres::{ Client, NoTls };
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };

mod models;
mod database;
use crate::models::response;



//senatize user data for security
//concurrency for multiple client connections
//Error handling where .unwrap_or_default() is used
//Log the errors somewhere
//possibly switch errors from string to &str

//Potentially connect user to db when making a conneciton for x amount of time so they don't need
//to keep reconnectiong if making multiple api calls

fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(models::DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            match database::queries::post::add_user(&mut client, &user) {
                Ok(_) => (response::OK_RESPONSE.to_string(), "User created".to_string()),
                Err(database::errors::PostError::InternalServerError(error)) => {
                    (response::INTERNAL_SERVER_ERROR.to_string(), error)
                }
            } 
        }
        _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(models::DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            match database::queries::get::get_user_by_id(&mut client, &id) {
                Ok(user) => {
                    (response::OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }                 
                Err(database::errors::GetError::UserNotFound) => {

                     (response::NOT_FOUND.to_string(), "User not found".to_string())
                }
                Err(database::errors::GetError::InternalServerError(error)) => {
                    (response::INTERNAL_SERVER_ERROR.to_string(), error)
                }             
            }
        }
        _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())       
    }
}

fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(models::DB_URL, NoTls) {
        Ok(mut client) => {
            match database::queries::get::get_all_users(&mut client) {
                Ok(users) => {
                    (response::OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
                }
                Err(database::errors::GetError::UserNotFound) => {
                    (response::NOT_FOUND.to_string(), "No users found".to_string())
                }
                Err(database::errors::GetError::InternalServerError(error)) => {
                    (response::INTERNAL_SERVER_ERROR.to_string(), error)
                }
            }
        }
        _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
    }
}

fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(), 
        get_user_request_body(&request),
        Client::connect(models::DB_URL, NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            match database::queries::put::update_user(&mut client, &user, &id) {
                Ok(_) => (response::OK_RESPONSE.to_string(), "User updated".to_string()),
                Err(database::errors::PutError::UserNotFound) => {
                    (response::NOT_FOUND.to_string(), "No users found".to_string())
                },
                Err(database::errors::PutError::InternalServerError(error)) => {
                    (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
                }
            } 
        }
        _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
    }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(models::DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            match database::queries::delete::delete_user(&mut client, &id) {
                Ok(_) => (response::OK_RESPONSE.to_string(), "User deleted".to_string()),
                Err(database::errors::DeleteError::UserNotFound) => {
                    (response::NOT_FOUND.to_string(), "User not found".to_string())
                }
                Err(database::errors::DeleteError::InternalServerError(error)) => {
                    (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
                }
            }
        }
        _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
    }
}

fn set_database() -> Result<(), InitializaitonError> {
    match Client::connect(models::DB_URL, NoTls) {
        Ok(mut client) => match database::queries::initialization::create_user_table(&mut client){
            Ok(_) => Ok(()),
            Err(InitializaitonError::InternalServerError(error)) => {
                Err(InitializaitonError::InternalServerError(error))
            }
        }
        _ => Err(InitializaitonError::InternalServerError("Failed to connect to database.".to_string()))
    }
}

fn get_id(request: &str) -> &str {
    request.split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

fn get_user_request_body(request: &str) -> Result<models::User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(r),
                r if r.starts_with("PUT /users/") => handle_put_request(r),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r),
                _ => (response::NOT_FOUND.to_string(), " 404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    if let Err(InitializaitonError::InternalServerError(error)) = set_database() {
        println!("Error: {}", error);
        return;
    }

    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(listener) => listener,
        Err(e) => {
            println!("Error binding to port 8080: {}", e);
            return;
        }
    };
    println!("Server started at port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error: {}", e),
        }
    }
}
