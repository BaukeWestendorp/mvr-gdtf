use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A file input/output error occurred: {0}")]
    Io(#[from] io::Error),

    #[error(
        "Could not open the archive at: '{path}': {source}. Please check if the file exists and you have permission to access it."
    )]
    OpenArchive { path: PathBuf, source: io::Error },
    #[error(
        "Could not unzip the archive at: '{path}': {source}. The file may be corrupted or not a valid archive."
    )]
    UnzipArchive { path: PathBuf, source: zip::result::ZipError },

    #[error("There was a problem parsing the general scene description XML: {source}")]
    ParseGeneralSceneDescription { source: quick_xml::DeError },

    #[error("The archive is missing a General Scene Description XML file: {source}")]
    MissingGeneralSceneDescriptionXml { source: zip::result::ZipError },

    #[error("Invalid file name: {0}")]
    InvalidFileName(String),
    #[error("Failed to parse matrix: {0}")]
    MatrixParseError(String),
    #[error("Invalid source type: {0}")]
    InvalidSourceType(String),
    #[error("Invalid color format: {0}")]
    InvalidColorFormat(String),
}
