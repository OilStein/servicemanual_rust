

use std::sync::Arc;
use surrealdb::sql::Value;
use surrealdb::{Datastore, Session};
use anyhow::{Result};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDB {
    pub ds: Arc<Datastore>,
    pub ses: Session
}

impl SurrealDB {
  pub async fn init() -> Result<SurrealDB> {
      let ds = Arc::new(Datastore::new("file://surreal.db").await?);
      
      let ses = Session::for_kv().with_ns("test").with_db("test");

      Ok(SurrealDB { ses, ds })
  }
}