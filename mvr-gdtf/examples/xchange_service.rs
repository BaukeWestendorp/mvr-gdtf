use std::{thread, time::Duration};

use uuid::Uuid;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let station_uuid = Uuid::parse_str("a4ef4b07-a6a4-4a60-ae6e-e6e5981a7427").unwrap();

    let service = mvr_gdtf::xchange::StationBuilder::new(
        "xchange-example".to_string(),
        "xchange-example".to_string(),
    )
    .station_uuid(station_uuid)
    .start()
    .unwrap();

    loop {
        for info in service.stations().unwrap() {
            eprintln!("{info:?}");
        }
        eprintln!("------------------");

        thread::sleep(Duration::from_secs_f32(3.0));
    }
}
