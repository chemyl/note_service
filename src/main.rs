use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;
mod routes;
mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(database_url).await
        .expect("Failed to connect to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
