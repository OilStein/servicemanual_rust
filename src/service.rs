use crate::{Machine, utils};
use crate::response::Response;
use actix_web::{web::Json, HttpResponse, post, get};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

pub type Services = Response<Service>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
  machine_id: String,
  service_id: String,
  notes: String,
  date: DateTime<Local>, // e.g. `2014-11-28T21:45:59.324310806+09:00`
  criticality: String,
  state: String
}


impl Service {
  pub fn new(m: String, n: String, c: String) -> Service {
    Service {
      machine_id: m,
      service_id: utils::generate_id(),
      notes: n,
      date: Local::now(),
      criticality: c,
      state: String::from("open"),
    }
  }

  pub fn close(&mut self){
    self.state = String::from("closed");
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceRequest {
  pub m_id: Option<String>,
  pub notes: String,
  pub criticality: String,
}

impl ServiceRequest {
    pub fn to_service(&self) -> Option<Service> {
      let m_id = self.m_id.clone()?;
      let notes = self.notes.clone();
      let c = self.criticality.clone();

      Some(Service::new(m_id, notes, c))
    }
}

#[post("/service")]
pub async fn create(service_req: Json<ServiceRequest>) -> HttpResponse {
  HttpResponse::Created().content_type("application/json").json(service_req.to_service())
}


#[get("/service")]
pub async fn list() -> HttpResponse {
  let service_calls = Services { results: vec![]};

  HttpResponse::Ok().content_type("application/json").json(service_calls)
}

