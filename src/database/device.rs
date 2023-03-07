use std::collections::BTreeMap;

use crate::device::Device;

use super::{Database, into_iter_objects};
use anyhow::{Result, anyhow};
use surrealdb::sql::Value;


impl Database {
  pub async fn select_all_device(&self) -> Result<()> {
    let sql = "SELECT * FROM machine WHERE model IS 'Type 1' ORDER BY name ASC LIMIT 5";
    let ress = self.ds.execute(sql, &self.ses, None, false).await?;
    for obj in into_iter_objects(ress)? {
      println!("{:?}", obj);
    }
    Ok(())
  }

    // Creates a device to the store. Returns created device id
    pub async fn create_device(&self, m: Device) -> Result<String> {
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
}


