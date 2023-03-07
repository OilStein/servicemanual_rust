use chrono::{DateTime, Local};

use crate::utils::generate_id;

#[derive(Debug)]
pub struct Maintenance {
  pub id: String,
  pub id_d: String, // relation to device
  pub desc: String, // description of task
  pub date: DateTime<Local>, // local time when task was registered 
  pub severity: String, // critical, important, unimportant
  pub status: String // open/closed
}

impl Maintenance {
    /// Create a new Maintenace obj. Assumes task to be open
    pub fn new(id_d: String, desc: String, severity: String) -> Self {
      Maintenance {
      id: generate_id(),
      id_d,
      desc,
      date: Local::now(), // System local time e.g. `2014-11-28T21:45:59.324310806+09:00`
      severity,
      status: String::from("open")
      }
    }

    // Closes the task
    // TODO Make db update call
    pub fn close(&mut self) {
      self.status = String::from("closed");
    }
}


#[cfg(test)]
mod tests {
  use crate::device::Device;

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
    let d = Device::new("Kone".to_string(), 2000, "Elevator".to_string());
    let id_d = &d.id; // Reference manipulation. Not sure is it good :D
    let mut t = Maintenance::new(id_d.to_string(), "Brake fault".to_string(), "unimportant".to_string());
    assert_eq!(&d.id, &t.id_d);
    t.close();
    assert_eq!(&t.status, &"closed".to_string());
  }
}