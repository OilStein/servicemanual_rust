
use crate::{utils, database::Database};
use anyhow::{Result};

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

  pub async fn read_csv_to_db(ds: &Database) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_path(r"D:\rust\servicemanual_rust\data\machines.csv")?;

    for result in reader.records() {
      let record = result?;
      let machine = Machine::new(
        record.get(0).ok_or_else(|| anyhow::anyhow!("missing name"))?.to_string(),
        record.get(1).ok_or_else(|| anyhow::anyhow!("missing year"))?.parse()?,
        record.get(2).ok_or_else(|| anyhow::anyhow!("missing model"))?.to_string(),
      );
      ds.create_machine(machine).await?;
    }
    Ok(())
  }
}

