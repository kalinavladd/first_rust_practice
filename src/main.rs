mod todolist;
use actix_web::{get, web, App, HttpServer, middleware::Logger};
use apalis::prelude::*;
use apalis::redis::RedisStorage;
use todolist::services;
use dotenv::dotenv;
use futures::future;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use todolist::tasks::send_email;
use anyhow::Result;
use apalis::layers::TraceLayer;

struct AppState {
    db: Pool<Postgres>,
}


#[get("/")]
async fn index() -> String {
    "This is health check".to_string()
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // connect db
    let database_url = std::env::var("DATABASE_URL").expect("database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error build connection pool");

    // connect redis
    let redis_url = std::env::var("REDIS_URL").expect("Failed get redis url");
    let storage = RedisStorage::connect(redis_url)
        .await.expect("Could not connect to redis storage");

    let data = web::Data::new(storage.clone());

    let http = async {
        HttpServer::new(move || {
            let logger = Logger::default();
            App::new()
                .wrap(logger)
                .app_data(web::Data::new(AppState { db: pool.clone(), }))
                .app_data(data.clone())
                .service(index)
                .configure(services::config)
        })
            .bind(("0.0.0.0", 8080))?
            .run()
            .await?;
            Ok(())
    };
    let worker = Monitor::new()
        .register_with_count(2, move |_| {
            WorkerBuilder::new(storage.clone())
                .layer(TraceLayer::new())
                .build_fn(send_email)
    })
    .run();

    future::try_join(http, worker).await.expect("failed");
    Ok(())
}
