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
pub(crate) fn load_zip<R: std::io::Read + std::io::Seek>(
    mut reader: R,
) -> Result<(uuid::Uuid, zip::ZipArchive<R>), crate::Error> {
    let mut file_bytes = Vec::new();
    reader.read_to_end(&mut file_bytes)?;

    let file_hash_uuid = {
        let hi = seahash::hash_seeded(&file_bytes, 0xdead, 0xbeef, 0xcafe, 0xbabe);
        let lo = seahash::hash_seeded(&file_bytes, 0x1234, 0x5678, 0x9abc, 0xdef0);
        uuid::Uuid::from_u64_pair(hi, lo)
    };

    let zip = zip::ZipArchive::new(reader).map_err(|e| crate::Error::UnzipArchive { source: e })?;

    Ok((file_hash_uuid, zip))
}

pub struct Resource {}
