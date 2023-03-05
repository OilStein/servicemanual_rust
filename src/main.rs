mod machines;
mod utils;
mod database;

use crate::{machines::Machine, database::Database};

use anyhow::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Database::new().await?;
 
    
    let _ma = Machine::read_csv_to_db(&ds).await?;
     ds.select_all_machine().await?; // works fine
    
    Ok(())
}
