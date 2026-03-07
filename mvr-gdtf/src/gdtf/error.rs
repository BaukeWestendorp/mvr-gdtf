#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The archive is missing a Description XML file: {source}")]
    MissingDescriptionXml { source: zip::result::ZipError },
    #[error("There was a problem parsing the description XML: {source}")]
    ParseDescription { source: quick_xml::DeError },

    #[error("Invalid data version string: '{misformatted_string}'")]
    InvalidDataVersion { misformatted_string: String },

    #[error("DMX address {raw_address} is out of bounds. Should be in the range of 1..=512")]
    DmxAddressOutOfBounds { raw_address: u16 },
    #[error("Failed to parse DMX address '{misformatted_string}'")]
    DmxAddressParseError { misformatted_string: String },

    #[error("DMX break value is zero, which is invalid")]
    DmxBreakZero,
    #[error("Failed to parse DMX break value")]
    DmxBreakParseError { misformatted_string: String },

    #[error("Physical value {raw_value} is out of bounds.")]
    PhysicalValueOutOfBounds { raw_value: f32 },

    #[error("Matrix format error: '{misformatted_string}'")]
    MatrixFormatError { misformatted_string: String },
    #[error("Matrix parse value error: '{misformatted_string}'")]
    MatrixParseValueError { misformatted_string: String },

    #[error("Invalid Name: '{misformatted_string}'")]
    NameInvalid { misformatted_string: String },

    #[error("Invalid Node: '{misformatted_string}'")]
    NodeInvalid { misformatted_string: String },

    #[error("DMX value bytes is zero (bytes: {bytes})")]
    DmxValueBytesZero { bytes: u8 },
    #[error("DMX value {value} out of bounds for {bytes} bytes (max: {max})")]
    DmxValueOutOfBounds { value: u32, bytes: u8, max: u32 },
    #[error("Failed to parse DMX value '{misformatted_string}'")]
    DmxValueParseError { misformatted_string: String },

    #[error("Failed to parse Vector3 value '{misformatted_string}'")]
    Vector3ParseValueError { misformatted_string: String },
    #[error("Failed to parse Vector3 format: '{misformatted_string}'")]
    Vector3FormatError { misformatted_string: String },

    #[error("Failed to parse DMX offset '{misformatted_string}'")]
    DmxOffsetParseError { misformatted_string: String },
}
