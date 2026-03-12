use uuid::Uuid;

fn main() {
    pretty_env_logger::init();

    let client = mvr_gdtf::xchange::Client::new(
        "Default".to_string(),
        "mvr-gdtf_xchange_client_example".to_string(),
        Uuid::new_v4(),
    )
    .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));

        for (uuid, info) in client.stations() {
            eprintln!("{uuid}: {info:?}");
        }
    }
}
