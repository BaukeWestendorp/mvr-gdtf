#[cfg(feature = "gdtf")]
pub mod gdtf;
#[cfg(feature = "mvr")]
pub mod mvr;
#[cfg(feature = "xchange")]
pub mod xchange;

mod error;
mod value;

pub use error::*;
#[cfg(any(feature = "mvr", feature = "gdtf"))]
pub use value::*;

#[cfg(any(feature = "mvr", feature = "gdtf"))]
pub(crate) fn load_zip(
    path: &std::path::Path,
) -> Result<(String, u64, uuid::Uuid, zip::ZipArchive<std::fs::File>), crate::Error> {
    let mut archive = std::fs::File::open(path)
        .map_err(|e| crate::Error::OpenArchive { source: e, path: path.to_path_buf() })?;

    let file_name = path.file_name().expect("Should be a file").to_string_lossy().to_string();
    // FIXME: We probably should not default to 0 byte file size.
    let file_size = archive.metadata().map(|meta| meta.len()).unwrap_or_default();
    let file_hash_uuid = {
        use std::io::Read as _;

        let mut file_bytes = vec![0; file_size as usize];
        archive.read_exact(&mut file_bytes)?;

        let hi = seahash::hash_seeded(&file_bytes, 0xdead, 0xbeef, 0xcafe, 0xbabe);
        let lo = seahash::hash_seeded(&file_bytes, 0x1234, 0x5678, 0x9abc, 0xdef0);

        uuid::Uuid::from_u64_pair(hi, lo)
    };

    let zip = zip::ZipArchive::new(archive)
        .map_err(|e| crate::Error::UnzipArchive { source: e, path: path.to_path_buf() })?;

    Ok((file_name, file_size, file_hash_uuid, zip))
}

pub struct Resource {}
