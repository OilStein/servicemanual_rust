

use std::collections::BTreeMap;

use actix_web::web::Data;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Value, Object, Array, thing};

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


  pub async fn create<T: Creatable>(db: Data<SurrealDB>, tb: &str, data: T) -> Result<Object, Error> {
    let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

    let data: Object = W(data.into()).try_into()?;

    let vars: BTreeMap<String, Value> = map![
      "tb".into() => tb.into(),
      "data".into() => Value::from(data),
    ];

    let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

    let val = res.into_iter().next().map(|r| r.result).expect("id not returned")?;

    W(val.first()).try_into()

  }

  pub async fn get(db: Data<SurrealDB>, id: &str) -> Result<Object, Error> {
    let sql = "SELECT * FROM $th";

    let d_id = format!("device:{}", id);

    let vars: BTreeMap<String, Value> = map![
      "th".into() => thing(&d_id)?.into()
    ];

    let res =  db.ds.execute(sql, &db.ses, Some(vars), true).await?;
    let obj = res.into_iter().next().expect("Failed to get response");

    W(obj.result?.first()).try_into()
  }

}