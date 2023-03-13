use chrono::{DateTime, Utc};

use crate::utils::id_generator::generate_id;

#[derive(Debug)]
pub struct Maintenance {
  pub id: String,
  pub id_d: String, // relation to device
  pub desc: String, // description of task
  pub date: DateTime<Utc>, // utc time. Surreal can take this value more easy
  pub severity: String, // critical, important, unimportant
  pub status: String // open/closed
}

impl Maintenance {
    /// Creates a new Maintenace obj. Assumes task to be open
    pub fn new(id_d: String, desc: String, severity: String) -> Self {
      Maintenance {
      id: generate_id(),
      id_d,
      desc,
      date: Utc::now(), // System local time e.g. `2014-11-28T21:45:59.324310806+09:00`
      severity,
      status: String::from("open")
      }
    }

    // Closes the task
    // TODO Make db update call
    pub fn close(&mut self) {
      self.status = String::from("closed");
    }

    /// Formats date to "01-01-2020 00:00:00"
    pub fn date_to_string(&self) -> String {
      format!("{}", self.date.format("%d-%m-%Y %H:%M:%S"))
    }

}


#[cfg(test)]
mod tests {
  use crate::model::device::Device;

use super::*;

  #[test]
  fn create_new_and_close_it() {
    let mut m = Maintenance::new("123".to_string(), "test".to_string(), "critical".to_string());
    assert_eq!((&m.id_d, &m.severity, &m.status), (&"123".to_string(), &"critical".to_string(), &"open".to_string()));
    m.close();
    assert_eq!(&m.status, &"closed".to_string())
  }

  #[test]
  fn create_device_and_task(){
    let d = &Device::new("Kone".to_string(), 2000, "Elevator".to_string());
    let mut t = Maintenance::new( d.id.to_string(), "Brake fault".to_string(), "unimportant".to_string());
    assert_eq!(&d.id, &t.id_d);
    t.close();
    assert_eq!(&t.status, &"closed".to_string());
  }
}