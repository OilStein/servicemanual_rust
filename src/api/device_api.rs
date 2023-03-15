use crate::database::surreal_db::SurrealDB;
use crate::models::device::{Device, DeviceBMC};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/devices")]
pub async fn create_device(db: Data<SurrealDB>, new_device: Json<Device>) -> HttpResponse {
    let data = Device {
        id: None,
        name: new_device.name.to_owned(),
        year: new_device.year.to_owned(),
        model: new_device.model.to_owned(),
    };

    let device_detail = DeviceBMC::create(db, "devices", data).await;

    match device_detail {
        Ok(device) => HttpResponse::Ok().json(device),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devices")]
pub async fn get_devices(db: Data<SurrealDB>) -> HttpResponse {
    let result = DeviceBMC::get_all(db).await;

    match result {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/devices/{id}")]
pub async fn get_device(db: Data<SurrealDB>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let device_detail = DeviceBMC::get(db, &id).await;

    match device_detail {
        Ok(device) => HttpResponse::Ok().json(device),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
