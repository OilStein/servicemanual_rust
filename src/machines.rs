use std::{error::Error};

use crate::utils;

#[derive(Debug)]
pub struct Machine {
  pub id: String,
  pub name: String,
  pub year: i32,
  pub model: String, // type == model
}

impl Machine {
  pub fn new(name: String, year: i32, model: String) -> Machine{
    Machine {
      id: utils::generate_id(),
      name,
      year,
      model
    }
  }

  pub fn read_csv_to_db() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(r"D:\rust\servicemanual_rust\data\machines.csv")?;

    for result in reader.records() {
      let _machine = result?;
    }
    Ok(())
  }
}

