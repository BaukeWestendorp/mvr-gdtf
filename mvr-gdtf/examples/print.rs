use mvr_gdtf::gdtf::GdtfFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    };

    let mvr_file = GdtfFile::load_from_file(file_path).unwrap();

    println!("{:#?}", mvr_file.general_scene_description());
}
