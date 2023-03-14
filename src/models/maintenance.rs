use std::collections::BTreeMap;

use actix_web::web::Data;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Array, Object, Value};

use crate::database::surreal_db::{Creatable, Patchable, SurrealDB};
use crate::prelude::*;
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub id: Option<String>,
    pub did: String,         // relation to device
    pub desc: String,        // description of task
    pub date: DateTime<Utc>, // utc time. Surreal can take this value more easy
    pub severity: String,    // critical, important, unimportant
    pub status: String,      // open/closed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenanceCreator {
    pub did: String,
    pub desc: String,
    pub severity: String,
}

impl Maintenance {
    pub fn new(did: String, desc: String, severity: String) -> Maintenance {
        Maintenance {
            id: None,
            did,
            desc,
            date: Utc::now(),
            severity,
            status: String::from("open"),
        }
    }
}

impl From<Maintenance> for Value {
    fn from(value: Maintenance) -> Self {
        match value.id {
            Some(v) => map![
              "id".into() => v.into(),
              "id_d".into() => value.did.into(),
              "desc".into() => value.desc.into(),
              "date".into() => value.date.into(),
              "severity".into() => value.severity.into(),
              "status".into() => value.status.into()
            ]
            .into(),
            None => map![
              "id_d".into() => value.did.into(),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MaintenancePatch {
    pub desc: Option<String>,
    pub severity: Option<String>,
    pub status: Option<String>,
}

impl From<MaintenancePatch> for Value {
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

pub struct MaintenanceBMC;

impl MaintenanceBMC {
    pub async fn get(db: Data<SurrealDB>, mid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";
        let mid = format!("device:{}", mid);

        let vars: BTreeMap<String, Value> = map![
          "th".into() => thing(&mid)?.into()
        ];

        let res = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
        let obj = res.into_iter().next().expect("Failed to get response");

        W(obj.result?.first()).try_into()
    }

    pub async fn get_all(db: Data<SurrealDB>) -> Result<Vec<Object>, Error> {
        let sql = "SELECT * FROM maintenance;";

        let res = db.ds.execute(sql, &db.ses, None, true).await?;

        let f_res = res.into_iter().next().expect("Failed to get response");

        let array: Array = W(f_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

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
            .expect("failded to return id")?;
        W(value.first()).try_into()
    }

    pub async fn update<T: Patchable>(
        db: Data<SurrealDB>,
        mid: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $date RETURN *";

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
