
use anyhow::{Result};

use crate::utils;
use crate::database::surreal_db::{SurrealDB};

#[derive(Debug)]
pub struct Device {
  pub id: String,
  pub name: String,
  pub year: i32,
  pub model: String, // type == model
}

impl Device {
  pub fn new(name: String, year: i32, model: String) -> Device{
    Device {
      id: utils::generate_id(),
      name,
      year,
      model
    }
  }


  // Naive reading. Doesn't check invalid data
  // TODO Figure out why relative path doesn't work 
  pub fn read_csv_to_db(ds: &SurrealDB) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(r"D:\rust\servicemanual_rust\data\devices.csv")?;

    for result in reader.records() {
      let record = result?;
      let device = Device::new(
        record.get(0).ok_or_else(|| anyhow::anyhow!("missing name"))?.to_string(),
        record.get(1).ok_or_else(|| anyhow::anyhow!("missing year"))?.parse()?,
        record.get(2).ok_or_else(|| anyhow::anyhow!("missing model"))?.to_string(),
      );
      // ds.create_device(device).await?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn device() {
    let mac = Device::new("Kone".to_string(), 2016, "Computer".to_string());
    assert_eq!((mac.name, mac.year, mac.model), ("Kone".to_string(), 2016, "Computer".to_string()))
  }
}