use std::{thread, time::Duration};

use mvr_gdtf::xchange::Settings;

fn main() {
    pretty_env_logger::init();

    let service = mvr_gdtf::xchange::Service::new(Settings {
        station_name: "xchange_example".to_string(),
        ..Default::default()
    })
    .unwrap();

    loop {
        for info in service.stations() {
            eprintln!("{info:?}");
        }
        eprintln!("------------------");

        thread::sleep(Duration::from_secs_f32(3.0));
    }
}
