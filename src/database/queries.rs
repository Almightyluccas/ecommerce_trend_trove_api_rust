use postgres::Client;
use crate::models;

pub mod get {
    use super::*;
    use crate::database::errors::GetError;

    pub fn get_user_by_id(client: &mut Client, user_id: &i32 ) -> Result<models::User, GetError> {
        match client.query("SELECT * FROM users WHERE id = $1", &[&user_id]) {
            Ok(rows) => match rows.iter().next() {
                    Some(row) => {
                        let user = models::User {
                            id: row.get(0),
                            name: row.get(1),
                            email: row.get(2),
                        };
                        Ok(user)
                    }
                    None => Err(GetError::UserNotFound),
            },
            Err(err) => Err(GetError::InternalServerError(err.to_string()))  ,
        }
    }
    pub fn get_all_users(client: &mut Client) -> Result<Vec<models::User>, GetError> {
        match client.query("SELECT * FROM users", &[]) {
            Ok(rows) => match rows.len() {
                0 => Err(GetError::UserNotFound),
                _ => {
                    let users: Vec<models::User> = rows
                        .iter()
                        .map(|row| models::User {
                            id: row.get(0),
                            name: row.get(1),
                            email: row.get(2)
                        })
                        .collect();
                    Ok(users)
                } 
            }
            Err(err) => Err(GetError::InternalServerError(err.to_string()))
        }
    } 
}

pub mod post {
    use super::*;
    use crate::database::errors::PostError;

    pub fn add_user(client: &mut Client ,new_user: &models::User) -> Result<(), PostError> {
       match client.query("INSERT INTO users (name, email) VALUES ($1, $2)", &[&new_user.name, &new_user.email]) {
           Ok(_) => Ok(()),
           Err(err) => Err(PostError::InternalServerError(err.to_string()))
       } 
    }
} 

pub mod put {
    use super::*;
    use crate::database::errors::PutError;

    pub fn update_user(client: &mut Client, user: &models::User, id: &i32) -> Result<(), PutError> {
        match client.query("UPDATE users SET name = $1, email = $2 WHERE id = $3",
                           &[&user.name, &user.email, &user.id]) {
            Ok(rows) => match rows.len() {
                0 => Err(PutError::UserNotFound),
                _ => Ok(())
            },
            Err(err) => Err(PutError::InternalServerError(err.to_string()))
        }
    }
}

pub mod delete {
    use super::*;
    use crate::database::errors::DeleteError;

    pub fn delete_user(client: &mut Client, id: &i32) -> Result<u64, DeleteError> {
        match client.execute("DELETE FROM users WHERE id = $1", &[&id]) {
            Ok(rows_affected) => {
                match rows_affected {
                    0 => Err(DeleteError::UserNotFound),
                    _ => Ok(rows_affected)
                }
            },
            Err(err) => Err(DeleteError::InternalServerError(err.to_string()))
        }
    }
}


pub mod initialization {
    use super::*;
    use crate::database::errors::InitializaitonError; 

    pub fn create_user_table(client: &mut Client) -> Result<(), InitializaitonError>{
        let query =  "CREATE TABLE IF NOT EXISTS users (
                        id SERIAL PRIMARY KEY,
                        name VARCHAR NOT NULL,
                        email VARCHAR NOT NULL
                      )";
        match client.query(query, &[]) {
            Ok(_) => Ok(()),
            Err(err) => Err(InitializaitonError::InternalServerError(err.to_string()))
        }


    } 
}



