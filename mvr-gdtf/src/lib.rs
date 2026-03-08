use std::{fs::File, path::Path};

use zip::ZipArchive;

#[cfg(feature = "gdtf")]
pub mod gdtf;
#[cfg(feature = "mvr")]
pub mod mvr;

mod error;
mod value;

pub use error::*;
pub use value::*;

#[cfg(any(feature = "mvr", feature = "gdtf"))]
pub(crate) fn load_zip(path: &Path) -> Result<ZipArchive<File>, crate::Error> {
    let archive = File::open(path)
        .map_err(|e| crate::Error::OpenArchive { source: e, path: path.to_path_buf() })?;

    let zip = zip::ZipArchive::new(archive)
        .map_err(|e| crate::Error::UnzipArchive { source: e, path: path.to_path_buf() })?;

    Ok(zip)
}

pub struct Resource {}
