use std::{
    fs,
    io::{self, Read as _},
    path::Path,
};

use uuid::Uuid;
use zip::ZipArchive;

use crate::{Resource, load_zip};

mod description;
mod error;
mod value;

pub use description::*;
pub use error::*;
pub use value::*;

pub struct GdtfFile {
    description: GdtfDescription,
    resources: Vec<Resource>,

    file_hash_uuid: Uuid,
}

impl GdtfFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let archive = fs::File::open(path).map_err(|e| crate::Error::OpenArchive { source: e })?;
        let (file_hash_uuid, mut zip) = load_zip(archive)?;

        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new(), file_hash_uuid })
    }

    pub fn load_from_bytes(bytes: &[u8]) -> Result<Self, crate::Error> {
        let (file_hash_uuid, mut zip) = load_zip(io::Cursor::new(bytes))?;

        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new(), file_hash_uuid })
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    pub fn file_hash_uuid(&self) -> Uuid {
        self.file_hash_uuid
    }
}

fn load_description<R: io::Read + io::Seek>(
    zip: &mut ZipArchive<R>,
) -> Result<GdtfDescription, crate::Error> {
    const FILE_NAME: &str = "description.xml";

    let mut xml_file = zip
        .by_name(FILE_NAME)
        .map_err(|e| crate::gdtf::Error::MissingDescriptionXml { source: e })?;

    let mut xml_string = String::new();
    xml_file.read_to_string(&mut xml_string)?;

    let gsd = quick_xml::de::from_str(&xml_string)
        .map_err(|e| crate::gdtf::Error::ParseDescription { source: e })?;

    Ok(gsd)
}
