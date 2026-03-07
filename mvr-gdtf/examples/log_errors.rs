use std::collections::HashMap;

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

    let mut versions = HashMap::new();

    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy(),
                None => continue,
            };

            let result = GdtfFile::load_from_file(&path);
            match result {
                Ok(gdtf_file) => {
                    let v = (
                        gdtf_file.description().data_version.major,
                        gdtf_file.description().data_version.minor,
                    );
                    versions.entry(v).or_insert_with(VersionData::default).count += 1;
                    let fixture_type_count = gdtf_file.description().fixture_types.len();
                    let entry = versions.entry(v).or_insert_with(VersionData::default);
                    if fixture_type_count > entry.max_fixture_type_count {
                        entry.max_fixture_type_count = fixture_type_count;
                    }
                }
                Err(e) => {
                    println!("{e} --- {}", file_name);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct VersionData {
    count: usize,
    max_fixture_type_count: usize,
}
