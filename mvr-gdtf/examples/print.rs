use mvr_gdtf::gdtf::GdtfFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let folder_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    };

    let entries = match std::fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory '{}': {}", folder_path, e);
            std::process::exit(1);
        }
    };
    let entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>().unwrap();
    let total = entries.len();
    let mut error_count = 0;
    for (idx, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        if path.is_file() {
            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy(),
                None => continue,
            };

            let result = GdtfFile::load_from_file(&path);
            match result {
                Ok(_) => {
                    // println!("\x1b[32m{:0>4}/{:0<4}:  OK - {}\x1b[0m", idx, total, file_name);
                }
                Err(e) => {
                    error_count += 1;
                    println!(
                        "\x1b[31m{:0>4}/{:0<4}: ERR - {} ({})\x1b[0m",
                        idx, total, file_name, e
                    );
                }
            }
        }
    }
    println!("Total errors: {}", error_count);
}
