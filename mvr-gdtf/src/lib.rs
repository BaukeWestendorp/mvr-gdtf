use std::{fs::File, path::Path};

use zip::ZipArchive;

pub mod gdtf;
pub mod mvr;

mod error;
mod value;

pub use error::*;
pub use value::*;

pub struct Resource {}

pub(crate) fn load_zip(path: &Path) -> Result<ZipArchive<File>, crate::Error> {
    let archive = File::open(path)
        .map_err(|e| crate::Error::OpenArchive { source: e, path: path.to_path_buf() })?;

    let zip = zip::ZipArchive::new(archive)
        .map_err(|e| crate::Error::UnzipArchive { source: e, path: path.to_path_buf() })?;

    Ok(zip)
}

pub(crate) fn deserialize_option_from_string_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    use serde::de::Deserialize as _;
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s == "None" => Ok(None),
        Some(s) => T::from_str(&s)
            .map(Some)
            .map_err(|e| serde::de::Error::custom(format!("Failed to parse: {}", e))),
    }
}

pub(crate) fn serialize_option_as_string_none<S, T>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: std::fmt::Display,
{
    match value {
        None => serializer.serialize_str("None"),
        Some(v) => serializer.serialize_str(&v.to_string()),
    }
}
