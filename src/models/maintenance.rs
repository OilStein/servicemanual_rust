use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
  pub id: Option<String>,
  pub id_d: String, // relation to device
  pub desc: String, // description of task
  pub date: DateTime<Utc>, // utc time. Surreal can take this value more easy
  pub severity: String, // critical, important, unimportant
  pub status: String // open/closed
}


