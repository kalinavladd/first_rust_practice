use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use apalis::prelude::*;


#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub text: String,
    pub subject: String,
}

impl Job for Email {
    const NAME: &'static str = "apalis::Email";
}


#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Article {
    pub id: i32,
    pub title:String,
    pub content: String,
    pub created_by: i32
}