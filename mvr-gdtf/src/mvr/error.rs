#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("There was a problem parsing the general scene description XML: {source}")]
    ParseGeneralSceneDescription { source: quick_xml::DeError },
    #[error("The archive is missing a General Scene Description XML file: {source}")]
    MissingGeneralSceneDescriptionXml { source: zip::result::ZipError },

    #[error("Matrix format error: '{misformatted_string}'")]
    MatrixFormatError { misformatted_string: String },
    #[error("Matrix parse value error: '{misformatted_string}'")]
    MatrixParseValueError { misformatted_string: String },

    #[error("Invalid file name: '{misformatted_name}'")]
    InvalidFileName { misformatted_name: String },
}
