use std::collections::BTreeMap;

use actix_web::web::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Array, Object, Value};

use crate::database::surreal_db::{Creatable, Patchable, SurrealDB};
use crate::prelude::*;
use crate::utils::macros::map;

/// Base struct for maintenance call
#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub id: Option<String>,
    pub did: String,         // relation to device
    pub desc: String,        // description of task
    pub date: DateTime<Utc>, // utc time. Surreal can take this value more easily
    pub severity: String,    // critical, important, unimportant
    pub status: String,      // open/closed
}

/// Struct for creating maintenence call. Important in Json reading in post call
#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceCreator {
    pub did: String,
    pub desc: String,
    pub severity: String,
}

impl Maintenance {
    /// Constructor
    pub fn new(did: String, desc: String, severity: String) -> Maintenance {
        Maintenance {
            id: None,
            did,
            desc,
            date: Utc::now(),
            severity,
            status: String::from("open"), // Always open state when creating a maintenance call
        }
    }
}

impl From<Maintenance> for Value {
    /// Converts Maintenance object to surrealdb Value
    fn from(value: Maintenance) -> Self {
        match value.id {
            Some(v) => map![
              "id".into() => v.into(),
              "did".into() => value.did.into(),
              "desc".into() => value.desc.into(),
              "date".into() => value.date.into(),
              "severity".into() => value.severity.into(),
              "status".into() => value.status.into()
            ]
            .into(),
            None => map![
              "did".into() => value.did.into(),
              "desc".into() => value.desc.into(),
              "date".into() => value.date.into(),
              "severity".into() => value.severity.into(),
              "status".into() => value.status.into()
            ]
            .into(),
        }
    }
}

impl Creatable for Maintenance {}

/// Struct for update calls. 
#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenancePatch {
    pub desc: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
}

impl From<MaintenancePatch> for Value {
    /// Makes surrealdb value with needed data
    fn from(value: MaintenancePatch) -> Self {
        let mut patch: BTreeMap<String, Value> = BTreeMap::new();

        if let Some(t) = value.desc {
            patch.insert("desc".into(), t.into());
        }

        if let Some(t) = value.severity {
            patch.insert("severity".into(), t.into());
        }

        if let Some(t) = value.status {
            patch.insert("status".into(), t.into());
        }

        Value::from(patch)
    }
}

impl Patchable for MaintenancePatch {}

/// Controller for CRUD functions
pub struct MaintenanceBMC;

impl MaintenanceBMC {
    /// Returns surrealdb Object from database, which is serializable as JSON
    pub async fn get(db: Data<SurrealDB>, mid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";
        let mid = format!("maintenance:{}", mid);

        let vars: BTreeMap<String, Value> = map![
          "th".into() => thing(&mid)?.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
        let obj = res.into_iter().next().expect("Failed to get response");

        W(obj.result?.first()).try_into()
    }
    /// Returns vector of Objects from database
    pub async fn get_all(db: Data<SurrealDB>) -> Result<Vec<Object>, Error> {
        let sql = "SELECT * FROM maintenance";

        let res = db.ds.execute(sql, &db.ses, None, true).await?;

        let f_res = res.into_iter().next().expect("Failed to get response");

        let array: Array = W(f_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    /// Creates Maintenance Object, posts it to db and returns object
    pub async fn create<T: Creatable>(
        db: Data<SurrealDB>,
        tb: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
          "tb".into() => tb.into(),
          "data".into() => Value::from(data),
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let value = res
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("failed to return id")?;
        W(value.first()).try_into()
    }

    // Updates object and returns that object
    pub async fn update<T: Patchable>(
        db: Data<SurrealDB>,
        mid: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *";

        let mid = format!("maintenance:{}", mid);

        let vars = map![
          "th".into() => thing(&mid)?.into(),
          "data".into() => data.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let f_res = res.into_iter().next().expect("id not returned");
        let result = f_res.result?;

        W(result.first()).try_into()
    }

    /// Deletes object from table
    pub async fn delete(db: Data<SurrealDB>, mid: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *";
        let mid = format!("maintenance:{}", mid);
        let vars = map![
          "th".into() => thing(&mid)?.into()
        ];
        let res = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
        let f_res = res.into_iter().next().expect("id not returned");
        f_res.result?;

        Ok(mid)
    }
}
