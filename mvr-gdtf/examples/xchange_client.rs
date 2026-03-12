use std::{thread, time::Duration};

use mvr_gdtf::xchange::Settings;

fn main() {
    pretty_env_logger::init();

    let client = mvr_gdtf::xchange::Client::new(Settings {
        station_name: "mvr-gdtf Client Example".to_string(),
        ..Default::default()
    })
    .expect("should create mvr-xchange client");

    loop {
        for (_uuid, info) in client.stations() {
            eprintln!("{info:?}");
        }
        eprintln!("------------------");

        thread::sleep(Duration::from_secs_f32(0.5));
    }
}
