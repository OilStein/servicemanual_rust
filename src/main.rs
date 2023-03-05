#![allow(unused)] // Remove for prod

mod machines;
mod service;
mod utils;
use crate::machines::Machine;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::{Result};
use surrealdb::{Datastore, Session};

type DB = (Datastore, Session);

#[actix_web::main]
async fn main() -> Result<()> {
    //let m = Machine::new("name".to_string(), 2000, "EV".to_string());

    //println!("{:?}", m);

   // let _ma = Machine::read_csv_to_db();

   let db: &DB = &(Datastore::new("memory").await?, Session::for_db("my_ns", "my_db"));
   let (ds, ses) = db;



    HttpServer::new(||{
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
