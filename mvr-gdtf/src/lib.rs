#[cfg(feature = "gdtf")]
pub mod gdtf;
#[cfg(feature = "mvr")]
pub mod mvr;
#[cfg(feature = "xchange")]
pub mod xchange;

mod error;
mod value;

pub use error::*;
pub use value::*;

#[cfg(any(feature = "mvr", feature = "gdtf"))]
pub(crate) fn load_zip(
    path: &std::path::Path,
) -> Result<zip::ZipArchive<std::fs::File>, crate::Error> {
    let archive = std::fs::File::open(path)
        .map_err(|e| crate::Error::OpenArchive { source: e, path: path.to_path_buf() })?;

    let zip = zip::ZipArchive::new(archive)
        .map_err(|e| crate::Error::UnzipArchive { source: e, path: path.to_path_buf() })?;

    Ok(zip)
}

pub struct Resource {}
