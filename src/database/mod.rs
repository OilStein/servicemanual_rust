use surrealdb::{Datastore, Session, sql::{Value, Object}, Response};
use anyhow::{anyhow, Result};

// references of this folders modules
mod device; 
mod maintenance;

pub struct Database {
  ds: Datastore,
  ses: Session,
}

impl Database {

  // Creates a new datastore
  pub async fn new() -> Result<Self>{
    let ds = Datastore::new("memory").await?;
    let ses = Session::for_db("ns", "db");
    Ok(Database{ds, ses})
  }
}

// Makes response iterable objects
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



// Test boilerplate
#[cfg(test)]
mod tests {
use super::*;

  #[tokio::test]
  async fn test() -> Result<()> {
    let _ds = Database::new().await?;
    Ok(())
  }

}
