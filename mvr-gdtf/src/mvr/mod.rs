use std::{
    fs,
    io::{self, Read as _},
    path::{Path, PathBuf},
};

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

    file_path: Option<PathBuf>,
    file_hash_uuid: Uuid,
}

impl MvrFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let archive = fs::File::open(path).map_err(|e| crate::Error::OpenArchive { source: e })?;
        let (file_hash_uuid, mut zip) = load_zip(archive)?;

        let general_scene_description = load_general_scene_description(&mut zip)?;

        Ok(Self {
            general_scene_description,
            gdtf_files: Vec::new(),
            resources: Vec::new(),
            file_path: Some(path.to_path_buf()),
            file_hash_uuid,
        })
    }

    pub fn load_from_bytes(
        bytes: &[u8],
        file_path: Option<impl Into<PathBuf>>,
    ) -> Result<Self, crate::Error> {
        let (file_hash_uuid, mut zip) = load_zip(io::Cursor::new(bytes))?;

        let general_scene_description = load_general_scene_description(&mut zip)?;

        Ok(Self {
            general_scene_description,
            gdtf_files: Vec::new(),
            resources: Vec::new(),
            file_path: file_path.map(Into::into),
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

    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    pub fn file_hash_uuid(&self) -> Uuid {
        self.file_hash_uuid
    }
}

fn load_general_scene_description<R: io::Read + io::Seek>(
    zip: &mut ZipArchive<R>,
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
