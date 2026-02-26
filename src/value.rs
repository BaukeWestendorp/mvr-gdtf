/// A file name that is valid according to the following rules:
/// - Case-sensitive
/// - Contains extension delimited by '.'
/// - Base name is not empty
/// - No FAT32 or NTFS reserved characters
/// - Only one '.'
/// - Only [A-Z], [a-z], [0-9], '_', '-', '.'
#[derive(facet::Facet, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[facet(transparent)]
pub struct FileName(String);

impl FileName {
    pub fn new(name: impl Into<String>) -> Result<Self, crate::Error> {
        let name = name.into();

        if name.is_empty() {
            return Err(crate::Error::InvalidFileName(
                "Filename cannot be empty".to_string(),
            ));
        }

        const RESERVED: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        if name.chars().any(|c| RESERVED.contains(&c)) {
            return Err(crate::Error::InvalidFileName(
                "Filename contains reserved characters".to_string(),
            ));
        }

        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
        {
            return Err(crate::Error::InvalidFileName(
                "Filename contains invalid characters".to_string(),
            ));
        }

        let dot_count = name.matches('.').count();
        if dot_count != 1 {
            return Err(crate::Error::InvalidFileName(
                "Filename must contain exactly one '.' separating base name and extension"
                    .to_string(),
            ));
        }

        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(crate::Error::InvalidFileName(
                "Filename must have a non-empty base name and extension".to_string(),
            ));
        }

        Ok(FileName(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
