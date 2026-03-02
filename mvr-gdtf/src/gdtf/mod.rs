mod desc;
mod values;

use std::{fs::File, io::Read as _, path::Path};

use zip::ZipArchive;

use crate::{Resource, load_zip};

pub use desc::*;
pub use values::*;

pub struct GdtfFile {
    description: GdtfDescription,
    resources: Vec<Resource>,
}

impl GdtfFile {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
        let path = path.as_ref();

        let mut zip = load_zip(path)?;
        let description = load_description(&mut zip)?;

        Ok(Self { description, resources: Vec::new() })
    }

    pub fn general_scene_description(&self) -> &GdtfDescription {
        &self.description
    }

    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }
}

fn load_description(zip: &mut ZipArchive<File>) -> Result<GdtfDescription, crate::Error> {
    const FILE_NAME: &str = "description.xml";

    let mut xml_file =
        zip.by_name(FILE_NAME).map_err(|e| crate::Error::MissingDescriptionXml { source: e })?;

    let mut xml_string = String::new();
    xml_file.read_to_string(&mut xml_string)?;

    let gsd = quick_xml::de::from_str(&xml_string)
        .map_err(|e| crate::Error::ParseDescription { source: e })?;

    Ok(gsd)
}
