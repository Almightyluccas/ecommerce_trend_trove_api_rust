use postgres::Client;
use crate::{database, models::response};

pub mod post {
    use super::*;
    use crate::utils::get_user_request_body;

    pub fn handle_register_request(request: &str, mut client: &Client) -> (String, String) {
        match get_user_request_body(&request) {
            Ok(user) => {
                match database::users::post::add_user(&mut client, &user) {
                    Ok(_) => (response::OK_RESPONSE.to_string(), "User created".to_string()),
                    Err(database::errors::PostError::InternalServerError(error)) => {
                        (response::INTERNAL_SERVER_ERROR.to_string(), error)
                    }
                } 
            }
            _ => (response::INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        }
    }
}

pub mod get {
    use super::*;
    use crate::utils::get_id;

    pub fn handle_get_request(request: &str, mut client: &Client) -> (String, String) {
        match get_id(&request).parse::<i32>(){
            Ok(id) => {
                match database::users::get::get_user_by_id(&mut client, &id) {
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

    pub fn handle_get_all_request(request: &str, mut client: &Client) -> (String, String) { 
        match database::users::get::get_all_users(&mut client) {
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
}

pub mod put {
    use super::*;
    use crate::utils::{ get_id, get_user_request_body};

    pub fn handle_put_request(request: &str, mut client: &Client) -> (String, String) {
        match (
            get_id(&request).parse::<i32>(), 
            get_user_request_body(&request),
        ) {
            (Ok(id), Ok(user)) => {
                match database::users::put::update_user(&mut client, &user, &id) {
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
}

pub mod delete {
    use super::*;
    use crate::utils::get_id;

    pub fn handle_delete_request(request: &str, mut client: &Client) -> (String, String) {
        match get_id(&request).parse::<i32>() {
            Ok(id) => {
                match database::users::delete::delete_user(&mut client, &id) {
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
}



