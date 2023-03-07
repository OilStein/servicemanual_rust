mod device;
mod utils;
mod database;
mod maintenance;

use crate::{device::Device, database::Database};

use anyhow::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Creates surrealdb datastore
    let ds = Database::new().await?;
 
    // Reads machines.csv file and stores Devices to datastore
    let _ma = Device::read_csv_to_db(&ds).await?;
     ds.select_all_device().await?; // works fine
    
    Ok(())
}
