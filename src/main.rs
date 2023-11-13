use database::errors::InitializaitonError;
use postgres::{ Client, NoTls };
use std::net::TcpListener;

mod models;
mod database;
mod http;
mod utils;

//senatize user data for security
//concurrency for multiple client connections
//Error handling where .unwrap_or_default() is used
//Log the errors somewhere
//possibly switch errors from string to &str

//Potentially connect user to db when making a conneciton for x amount of time so they don't need
//to keep reconnectiong if making multiple api calls

fn set_database() -> Result<(), InitializaitonError> {
    match Client::connect(models::DB_URL, NoTls) {
        Ok(mut client) => match database::users::initialization::create_user_table(&mut client){
            Ok(_) => Ok(()),
            Err(InitializaitonError::InternalServerError(error)) => {
                Err(InitializaitonError::InternalServerError(error))
            }
        }
        _ => Err(InitializaitonError::InternalServerError("Failed to connect to database.".to_string()))
    }
}




fn main() {
    if let Err(InitializaitonError::InternalServerError(error)) = set_database() {
        println!("Error: {}", error);
        return;
    }
    let database_client = match Client::connect(models::DB_URL, NoTls) {
        Ok(client) => client,
        Err(e) => {
            println!("Database connection error: {}", e);
            None
        }
    };

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
            let client = database_client.as_ref();
            Ok(stream) => http::routes::handle_client(stream, client,
            Err(e) => println!("Error: {}", e),
        }
    }
}
