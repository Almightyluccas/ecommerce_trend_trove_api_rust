//Refactor to have reponse instead of just error so the returned value is known
//handle different types of errors for better error messages
//rename UserNotFound to NotFound

#[derive(Debug)]
pub enum GetError {
    UserNotFound,
    InternalServerError(String),
}
#[derive(Debug)]
pub enum PutError {
    UserNotFound,
    InternalServerError(String),
}
#[derive(Debug)] 
pub enum DeleteError {
    UserNotFound,
    InternalServerError(String),
}
#[derive(Debug)]
pub enum PostError {
    InternalServerError(String),
}

#[derive(Debug)] 
pub enum InitializaitonError {
    InternalServerError(String),
}
