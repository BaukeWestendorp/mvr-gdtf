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

    #[error("The archive is missing a Description XML file: {source}")]
    MissingDescriptionXml { source: zip::result::ZipError },
    #[error("There was a problem parsing the description XML: {source}")]
    ParseDescription { source: quick_xml::DeError },

    #[error("CIE color parse X error: {0}")]
    CieColorParseXError(String),
    #[error("CIE color parse Y error: {0}")]
    CieColorParseYError(String),
    #[error("CIE color parse YY error: {0}")]
    CieColorParseYYError(String),
    #[error("Invalid CIE color: {0}")]
    InvalidCieColor(String),

    #[error("Invalid file name: {0}")]
    InvalidFileName(String),

    #[error("Matrix format error: {0}")]
    MatrixFormatError(String),
    #[error("Matrix parse value error: {0}")]
    MatrixParseValueError(String),

    #[error("DMX break value cannot be zero.")]
    DmxBreakZero,
    #[error("DMX break parse error: {0}")]
    DmxBreakParseError(String),
    #[error("Invalid DMX break format: {0}")]
    InvalidDmxBreakFormat(String),
    #[error("Invalid DMX address: {0}")]
    InvalidDmxAddress(String),
    #[error("Invalid physical value: {0}")]
    InvalidPhysicalValue(f32),

    #[error("Invalid GDTF Name: {0}")]
    InvalidName(String),
    #[error("Invalid GDTF Node reference: {0}")]
    InvalidNode(String),
    #[error("Invalid GDTF FeatureType (expected Group.Feature): {0}")]
    InvalidFeatureType(String),
    #[error("Invalid GDTF DataVersion (expected major.minor): {0}")]
    InvalidDataVersion(String),
    #[error("Invalid GDTF Vector3 (expected x,y,z): {0}")]
    InvalidVector3(String),
    #[error("Invalid GDTF TwoArray (expected x,y): {0}")]
    InvalidTwoArray(String),
    #[error("Invalid GDTF Offset (expected None or n[,n]*): {0}")]
    InvalidOffset(String),
    #[error("Invalid GDTF DMX value (expected None or Uint/n or Uint/ns): {0}")]
    InvalidDmxValue(String),
}
