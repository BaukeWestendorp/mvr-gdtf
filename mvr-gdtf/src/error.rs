use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[cfg(feature = "gdtf")]
    #[error("GDTF error: {0}")]
    Gdtf(#[from] crate::gdtf::Error),
    #[cfg(feature = "mvr")]
    #[error("MVR error: {0}")]
    Mvr(#[from] crate::mvr::Error),

    #[error(
        "Could not open the archive at: '{path}': {source}. Please check if the file exists and you have permission to access it."
    )]
    OpenArchive { path: PathBuf, source: io::Error },

    #[error(
        "Could not unzip the archive at: '{path}': {source}. The file may be corrupted or not a valid archive."
    )]
    UnzipArchive { path: PathBuf, source: zip::result::ZipError },

    #[error("CIE color parse X error: '{misformatted_string}'")]
    CieColorParseXError { misformatted_string: String },
    #[error("CIE color parse Y error: '{misformatted_string}'")]
    CieColorParseYError { misformatted_string: String },
    #[error("CIE color parse YY error: '{misformatted_string}'")]
    CieColorParseYYError { misformatted_string: String },
    #[error("Invalid CIE color: '{misformatted_color}'")]
    InvalidCieColor { misformatted_color: String },
}
