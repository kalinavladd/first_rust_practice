use actix_web::{get, post, delete, put, web, Responder, HttpResponse, services};
use crate::{AppState};
use super::models::{CreateUser, User, Article, Email};
use sqlx::{self, FromRow};
use apalis::prelude::*;
use apalis::redis::RedisStorage;


pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(fetch_users)
        .service(push_email)
        .service(create_user);

    cfg.service(scope);
}

#[post("/email")]
async fn push_email(
    email: web::Json<Email>,
    storage: web::Data<RedisStorage<Email>>) -> HttpResponse {

    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(email.into_inner()).await;
    match res {
        Ok(()) => HttpResponse::Ok().body("Email added to queue".to_string()),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}


#[get("/users")]
async fn fetch_users(state: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Users list is empty")
    }

}

#[post("/users")]
async fn create_user(state: web::Data<AppState>, body: web::Json<CreateUser>) -> impl Responder {

    match sqlx::query_as::<_, User>(
        "insert into users(first_name, last_name) values ($1, $2) returning *"
    )
        .bind(body.first_name.to_string())
        .bind(body.last_name.to_string())
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => { HttpResponse::NotFound().json("Failed")
        },
    }
}
