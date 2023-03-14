use crate::database::surreal_db::SurrealDB;
use crate::models::maintenance::{
    Maintenance, MaintenanceBMC, MaintenanceCreator, MaintenancePatch,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/maintenances")]
pub async fn create_maintenance(
    db: Data<SurrealDB>,
    new_m: Json<MaintenanceCreator>,
) -> HttpResponse {
    let data: Maintenance = Maintenance::new(
        new_m.did.to_owned(),
        new_m.desc.to_owned(),
        new_m.severity.to_owned(),
    );

    let maintenance_detail = MaintenanceBMC::create(db, "maintenance", data).await;

    match maintenance_detail {
        Ok(maintenance) => HttpResponse::Ok().json(maintenance),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/maintenances")]
pub async fn get_maintenances(db: Data<SurrealDB>) -> HttpResponse {
    let result = MaintenanceBMC::get_all(db).await;

    match result {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/maintenances/{id}")]
pub async fn get_maintenance(db: Data<SurrealDB>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let maintenance_detail = MaintenanceBMC::get(db, &id).await;

    match maintenance_detail {
        Ok(maintenance) => HttpResponse::Ok().json(maintenance),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/maintenances/{id}")]
pub async fn update_maintenance(
    db: Data<SurrealDB>,
    path: Path<String>,
    patch: Json<MaintenancePatch>,
) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid id");
    };

    let data = MaintenancePatch {
        severity: patch.severity.to_owned(),
        desc: patch.desc.to_owned(),
        status: patch.status.to_owned(),
    };

    let update_result = MaintenanceBMC::update(db, &id, data).await;

    match update_result {
        Ok(maintenance) => HttpResponse::Ok().json(maintenance),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/maintenance/{id}")]
pub async fn delete_maintenance(db: Data<SurrealDB>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid id");
    };

    let result = MaintenanceBMC::delete(db, &id).await;

    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
