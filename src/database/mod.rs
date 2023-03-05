use std::{collections::BTreeMap};

use surrealdb::{Datastore, Session, sql::{Value, Object}, Response};
use anyhow::{anyhow, Result};

use crate::machines::Machine;

pub struct Database {
  ds: Datastore,
  ses: Session,
}

impl Database {
  pub async fn new() -> Result<Self>{
    let ds = Datastore::new("memory").await?;
    let ses = Session::for_db("ns", "db");
    Ok(Database { ds, ses})
  }

  pub async fn create_machine(&self, m: Machine) -> Result<String> {
    let sql = "CREATE machine CONTENT $data";

    let data: BTreeMap<String, Value> = [
      ("id".into(), m.id.into()),
      ("name".into(), m.name.into()),
      ("year".into(), m.year.into()),
      ("model".into(), m.model.into())
    ].into();

    let vars: BTreeMap<String, Value> = [
      ("data".into(), data.into())
    ].into();

    let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;

    into_iter_objects(ress)?
      .next()
      .transpose()?
      .and_then(|obj| obj.get("id").map(|id| id.to_string()))
      .ok_or_else(|| anyhow!("No id returned"))
  }

  pub async fn select_all_machine(&self) -> Result<()> {
    let sql = "SELECT * FROM machine WHERE model IS 'Type 1' ORDER BY name ASC LIMIT 5";
    let ress = self.ds.execute(sql, &self.ses, None, false).await?;
    for obj in into_iter_objects(ress)? {
      println!("{:?}", obj);
    }
    Ok(())
  }
    
}

pub fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
	let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

	match res {
		Some(Value::Array(arr)) => {
			let it = arr.into_iter().map(|v| match v {
				Value::Object(object) => Ok(object),
				_ => Err(anyhow!("A record was not an Object")),
			});
			Ok(it)
		}
		_ => Err(anyhow!("No records found.")),
	}
}