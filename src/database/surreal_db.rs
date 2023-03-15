use std::sync::Arc;
use surrealdb::sql::Value;
use surrealdb::{Datastore, Error, Session};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDB {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}


impl SurrealDB {
    /// Creates SurrealDB database wrapped in an atomic reference counter for cloning
    pub async fn init() -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new("memory").await?);
        let ses = Session::for_kv().with_ns("test").with_db("test");
        Ok(SurrealDB { ses, ds })
    }
}
