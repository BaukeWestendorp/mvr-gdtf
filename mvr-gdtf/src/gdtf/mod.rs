use std::{
    fs,
    io::{self, Read as _},
    path::{Path, PathBuf},
};

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

    file_path: Option<PathBuf>,
}

impl GdtfFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let archive = fs::File::open(path).map_err(|e| crate::Error::OpenArchive { source: e })?;
        let (_, mut zip) = load_zip(archive)?;

        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new(), file_path: Some(path.to_path_buf()) })
    }

    pub fn load_from_bytes(bytes: &[u8], file_path: Option<PathBuf>) -> Result<Self, crate::Error> {
        let (_, mut zip) = load_zip(io::Cursor::new(bytes))?;

        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new(), file_path })
    }

    pub fn description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
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
