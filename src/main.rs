mod utils;
mod database;
mod prelude;
mod error;
mod models;


use actix_web::{HttpServer, App, web::Data};

use crate::database::surreal_db::SurrealDB;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDB::init().await;
    let db_data = Data::new(surreal);

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
    })
    .bind("localhost:8080")?
    .run()
    .await
}
