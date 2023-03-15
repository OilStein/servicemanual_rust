use actix_web::web::Data;
use std::{env::current_dir, path::PathBuf};

use crate::{
    database::surreal_db::SurrealDB,
    models::{
        device::{Device, DeviceBMC},
        maintenance::{Maintenance, MaintenanceBMC},
    },
};

/// Populates SurrealDB with devices
pub async fn populate(db: Data<SurrealDB>) -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = [current_dir()?, "data".into(), "devices.csv".into()]
        .iter()
        .collect();
    // println!("{:?}", path.display());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    for result in reader.records() {
        let record = result?;
        let device = Device {
            id: None,
            name: record.get(0).ok_or("missing name")?.to_string(),
            year: record.get(1).ok_or("missing year")?.parse()?,
            model: record.get(2).ok_or("missing model")?.to_string(),
        };
        DeviceBMC::create(db.to_owned(), "device", device).await?;
    }

    let vec_of_device = DeviceBMC::get_all(db.to_owned()).await?;

    for _n in 0..=10 {
        let i = fastrand::usize(..vec_of_device.len());
        let element = vec_of_device.get(i).expect("msg");
        let value = element.get("id");

        match value {
            Some(id) => {
                let did = id.to_string();
                // println!("{}", did) Print correct id strings
                let desc = format!("Notes-{}", fastrand::i32(..100));
                let severity_vec = vec!["critical", "important", "unimportant"];
                let severity = severity_vec[fastrand::usize(..severity_vec.len())].to_string();
                let data = Maintenance::new(did, desc, severity);
                MaintenanceBMC::create(db.to_owned(), "maintenance", data).await?;
            }
            None => todo!(),
        }
    }

    Ok(())
}
