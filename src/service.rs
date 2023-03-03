use crate::Machine;
use chrono::prelude::*;

pub struct Service {
  machine: Machine,
  notes: String,
  date: DateTime<Local>, // e.g. `2014-11-28T21:45:59.324310806+09:00`
  criticality: String,
  state: Option<Box<dyn State>>,
}

pub enum Crit {
  Critical,
  Important,
  Minor
}
trait State {
  fn request_closing(self: Box<Self>) -> Box<dyn State>;
}

struct Open {
}
impl State for Open {
  fn request_closing(self: Box<Self>) -> Box<dyn State> {
    Box::new(Closed {})
  }
}
struct Closed{}

impl State for Closed {
    fn request_closing(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl Service {
  pub fn new(m: Machine, c: String) -> Service {
    Service {
      machine: m,
      notes: String::new(),
      date: Local::now(),
      criticality: c,
      state: Some(Box::new(Open{}))
    }
  }
  pub fn add_notes(&mut self, note: &str){
    self.notes.push_str(note);
  }

  pub fn request_closing(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.request_closing());
      }
  }

}


