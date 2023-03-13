

use std::collections::BTreeMap;

use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Value, Object, Array};

use crate::{database::surreal_db::{Creatable, SurrealDB}, utils::macros::map, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
  pub id: Option<String>,
  pub name: String,
  pub year: i32,
  pub model: String, // type == model
}

impl From<Device> for Value {
  fn from(val: Device) -> Self {
    match val.id {
        Some(v) => {
          map![
            "id".into() => v.into(),
            "name".into() => val.name.into(),
            "year".into() => val.year.into(),
            "model".into() => val.model.into(),
          ].into()
        },
        None => {
          map![
          "name".into() => val.name.into(),
          "year".into() => val.year.into(),
          "model".into() => val.model.into(),
          ].into()
        }
    }
  }
}

impl Creatable for Device{}

pub struct DeviceBMC;

impl DeviceBMC {
  pub async fn get_all(db: Data<SurrealDB>) -> Result<Vec<Object>, Error>{
    let sql = "SELECT * FROM device;";

    let res = db.ds.execute(sql, &db.ses, None, false).await?;
    let f_res = res.into_iter().next().expect("Did not get a response");
    let array: Array = W(f_res.result?).try_into()?;

    array.into_iter().map(|value| W(value).try_into()).collect()
     
  }
}