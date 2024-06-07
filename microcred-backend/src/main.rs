use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod api;
mod db;
mod linera;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_db_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set");
    let mongo_db = db::MongoDB::init(&mongo_db_uri).await;
    let mongo_data = web::Data::new(Arc::new(mongo_db));

    HttpServer::new(move || {
        App::new()
            .app_data(mongo_data.clone())
            .configure(api::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
