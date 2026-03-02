use std::{fs::File, path::Path};

use zip::ZipArchive;

pub mod gdtf;
pub mod mvr;

mod error;

pub use error::*;

pub struct Resource {}

pub(crate) fn load_zip(path: &Path) -> Result<ZipArchive<File>, crate::Error> {
    let archive = File::open(path)
        .map_err(|e| crate::Error::OpenArchive { source: e, path: path.to_path_buf() })?;

    let zip = zip::ZipArchive::new(archive)
        .map_err(|e| crate::Error::UnzipArchive { source: e, path: path.to_path_buf() })?;

    Ok(zip)
}
