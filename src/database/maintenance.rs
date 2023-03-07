use std::collections::BTreeMap;

use anyhow::{Result, anyhow};
use surrealdb::sql::{Value, thing};

use crate::maintenance::Maintenance;
use super::{Database, into_iter_objects};

impl Database {

  pub async fn create_maintenance(&self, m: Maintenance) -> Result<String> {
    let sql = "CREATE maintenance CONTENT $data";

    let data: BTreeMap<String, Value> = [
      ("id".into(), m.id.into()),
      ("id_d".into(), m.id_d.into()),
      ("desc".into(), m.desc.into()),
      ("date".into(), m.date.into()),
      ("severity".into(), m.severity.into()),
      ("status".into(), m.status.into())
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

  pub async fn select_all_maintenance(&self) -> Result<()>{
    // TODO Check with more data how ordering works
    let sql = "SELECT * FROM maintenance ORDER BY severity ASC";

    let ress = self.ds.execute(sql, &self.ses, None, false).await?;
    for obj in into_iter_objects(ress)? {
      println!("{}", obj?);
    }
    Ok(())
  }

}
    
