use mvr_rs::MvrFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    };

    let mvr_file = MvrFile::load_from_file(file_path).unwrap();
    let mut fixtures: Vec<_> = mvr_file
        .general_scene_description()
        .scene()
        .layers()
        .iter()
        .flat_map(|l| l.fixtures())
        .collect();

    fixtures.sort_by_key(|fixture| fixture.fixture_id());

    for fixture in fixtures {
        println!(
            "{:?}\t{:?}\t{:?}\t{}",
            fixture.fixture_id(),
            fixture.fixture_id_numeric(),
            fixture.unit_number(),
            fixture.name().to_string()
        );
    }
}
