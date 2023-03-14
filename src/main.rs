mod api;
mod database;
mod error;
mod models;
mod prelude;
mod utils;

use actix_web::{web::Data, App, HttpServer};

use crate::database::surreal_db::SurrealDB;
use api::device_api::*;
use api::maintenance_api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDB::init()
        .await
        .expect("Error connecting to SurrealDB");

    let db_data = Data::new(surreal);
    utils::populate::populate(db_data.to_owned())
        .await
        .expect("something");

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_device)
            .service(get_devices)
            .service(create_device)
            .service(get_maintenance)
            .service(get_maintenances)
            .service(create_maintenance)
            .service(delete_maintenance)
            .service(update_maintenance)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
