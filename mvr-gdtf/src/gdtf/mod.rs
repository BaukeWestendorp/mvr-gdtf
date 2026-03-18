use std::{fs::File, io::Read as _, path::Path};

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

    file_name: String,
    file_size: u64,
    file_hash_uuid: Uuid,
}

impl GdtfFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let (file_name, file_size, file_hash_uuid, mut zip) = load_zip(path)?;
        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new(), file_name, file_size, file_hash_uuid })
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn file_hash_uuid(&self) -> Uuid {
        self.file_hash_uuid
    }
}

fn load_description(zip: &mut ZipArchive<File>) -> Result<GdtfDescription, crate::Error> {
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
