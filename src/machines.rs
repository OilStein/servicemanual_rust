use std::{ collections::BTreeMap};

use anyhow::{anyhow, Result};
use surrealdb::{sql::{Value, Object}, Response};

use crate::{utils, DB};

#[derive(Debug)]
pub struct Machine {
  // id: String,
  name: String,
  year: i32,
  model: String, // type == model
}

impl Machine {
  pub fn new(name: String, year: i32, model: String) -> Machine{
    Machine {
      // id: utils::generate_id(),
      name,
      year,
      model
    }
  }

  pub fn read_csv_to_db() -> Result<()> {
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(r"D:\rust\servicemanual_rust\data\machines.csv")?;

    for result in reader.records() {
      let _machine = result?;
    }
    Ok(())
  }

  pub async fn create_machine((ds, ses): &DB, m: Machine) -> Result<String> {
    let sql = "CREATE machine CONTENT $data";

    let data: BTreeMap<String, Value> = [
      ("name".into(), m.name.into()),
      ("year".into(), m.year.into()),
      ("model".into(), m.model.into())
    ].into();

    let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

    let ress = ds.execute(sql, ses, Some(vars), false).await?;

    Self::into_iter_objects(ress).await?
      .next()
      .transpose()?
      .and_then(|obj| obj.get("id").map(|id| id.to_string()))
      .ok_or_else(|| anyhow!("No id returned."))
  }

  async fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>>  {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
          let it = arr.into_iter().map(|v| match v {
            Value::Object(object) => Ok(object),
            _ => Err(anyhow!("A record was not an Object!"))
          });
          Ok(it)
        }
        _ => Err(anyhow!("No record found!"))
    }

  }
}

