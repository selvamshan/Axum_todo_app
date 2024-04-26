use serde::{Deserialize, Serialize};

pub mod create_users;
pub mod login;

#[derive(Serialize, Deserialize)]
pub struct ResponseOutUser {
    data: ResponseUser
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token:String,
}

#[derive(Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}