use std::{fs::File, io::Read as _, path::Path};

use uuid::Uuid;
use zip::ZipArchive;

use crate::{Resource, gdtf::GdtfFile, load_zip};

mod error;
mod gsd;
mod value;

pub use error::*;
pub use gsd::*;
pub use value::*;

pub struct MvrFile {
    general_scene_description: GeneralSceneDescription,
    gdtf_files: Vec<GdtfFile>,
    resources: Vec<Resource>,

    file_name: String,
    file_size: u64,
    file_hash_uuid: Uuid,
}

impl MvrFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let (file_name, file_size, file_hash_uuid, mut zip) = load_zip(path)?;
        let general_scene_description = load_general_scene_description(&mut zip)?;

        Ok(Self {
            general_scene_description,
            gdtf_files: Vec::new(),
            resources: Vec::new(),
            file_name,
            file_size,
            file_hash_uuid,
        })
    }

    pub fn general_scene_description(&self) -> &GeneralSceneDescription {
        &self.general_scene_description
    }

    pub fn gdtf_files(&self) -> &[GdtfFile] {
        &self.gdtf_files
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

fn load_general_scene_description(
    zip: &mut ZipArchive<File>,
) -> Result<GeneralSceneDescription, crate::Error> {
    const FILE_NAME: &str = "GeneralSceneDescription.xml";

    let mut xml_file = zip
        .by_name(FILE_NAME)
        .map_err(|e| crate::mvr::Error::MissingGeneralSceneDescriptionXml { source: e })?;

    let mut xml_string = String::new();
    xml_file.read_to_string(&mut xml_string)?;

    let gsd = quick_xml::de::from_str(&xml_string)
        .map_err(|e| crate::mvr::Error::ParseGeneralSceneDescription { source: e })?;

    Ok(gsd)
}
