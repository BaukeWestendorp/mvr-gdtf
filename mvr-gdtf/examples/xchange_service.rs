use std::{thread, time::Duration};

use mvr_gdtf::xchange::Settings;
use uuid::Uuid;

fn main() {
    pretty_env_logger::init();

    let station_uuid = Uuid::parse_str("a4ef4b07-a6a4-4a60-ae6e-e6e5981a7427").unwrap();

    let service = mvr_gdtf::xchange::Service::new(Settings {
        station_name: "xchange_example".to_string(),
        station_uuid,
        ..Default::default()
    })
    .unwrap();

    service.start().unwrap();

    loop {
        for info in service.stations_in_group() {
            eprintln!("{info:?}");
        }
        eprintln!("------------------");

        thread::sleep(Duration::from_secs_f32(3.0));
    }
}
