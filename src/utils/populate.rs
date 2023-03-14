use actix_web::web::Data;

use crate::{
    database::surreal_db::SurrealDB,
    models::device::{Device, DeviceBMC},
};

pub async fn populate(db: Data<SurrealDB>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("D:/rust/servicemanual_rust/data/devices.csv")?;

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

    Ok(())
}
