use uuid::Uuid;

fn main() {
    let mut client = mvr_gdtf::xchange::Client::new(
        "Default".to_string(),
        "mvr-gdtf_xchange_client_example".to_string(),
        Uuid::new_v4(),
    )
    .unwrap();

    client.start();

    std::thread::sleep(std::time::Duration::from_secs(100000));
}
