mod device;
mod utils;
mod database;
mod maintenance;

use crate::{device::Device, database::Database};

use anyhow::{Result};
use maintenance::Maintenance;

#[tokio::main]
async fn main() -> Result<()> {
    // Creates surrealdb datastore
    let ds = Database::new().await?;
 
    // Reads machines.csv file and stores Devices to datastore
    let _ma = Device::read_csv_to_db(&ds).await?;
    ds.select_all_device().await?; // works fine

    // TODO Make ds.select_device_by_id()

    let task = Maintenance::new("123".to_string(), "Test".to_string(), "critical".to_string());
    let _ress = ds.create_maintenance(task).await?;

    ds.select_all_maintenance().await?; 


    Ok(())
}
