use std::fmt;
use std::str;

/// A file name that is valid according to the following rules:
/// - Case-sensitive
/// - Contains extension delimited by '.'
/// - Base name is not empty
/// - No FAT32 or NTFS reserved characters
/// - Only one '.'
/// - Only [A-Z], [a-z], [0-9], '_', '-', '.'
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FileName(String);

impl FileName {
    pub fn new(name: impl Into<String>) -> Result<Self, crate::Error> {
        let name = name.into();

        if name.is_empty() {
            return Err(crate::Error::InvalidFileName("Filename cannot be empty".to_string()));
        }

        const RESERVED: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        if name.chars().any(|c| RESERVED.contains(&c)) {
            return Err(crate::Error::InvalidFileName(
                "Filename contains reserved characters".to_string(),
            ));
        }

        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.') {
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CieColor {
    pub x: f32,
    pub y: f32,
    pub yy: f32,
}

impl fmt::Display for CieColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6},{:.6},{:.6}", self.x, self.y, self.yy)
    }
}

impl str::FromStr for CieColor {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(crate::Error::InvalidColorFormat(
                "CIE xyY string must not be empty".to_string(),
            ));
        }

        if s.starts_with(',') || s.ends_with(',') {
            return Err(crate::Error::InvalidColorFormat(
                "CIE xyY string must not have leading or trailing comma".to_string(),
            ));
        }

        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(crate::Error::InvalidColorFormat(
                "CIE xyY string must have three comma-separated values".to_string(),
            ));
        }
        let x = parts[0]
            .trim()
            .parse::<f32>()
            .map_err(|_| crate::Error::InvalidColorFormat("Failed to parse x value".to_string()))?;
        let y = parts[1]
            .trim()
            .parse::<f32>()
            .map_err(|_| crate::Error::InvalidColorFormat("Failed to parse y value".to_string()))?;
        let yy = parts[2]
            .trim()
            .parse::<f32>()
            .map_err(|_| crate::Error::InvalidColorFormat("Failed to parse Y value".to_string()))?;

        Ok(CieColor { x, y, yy })
    }
}

impl From<CieColor> for String {
    fn from(c: CieColor) -> Self {
        c.to_string()
    }
}

impl TryFrom<String> for CieColor {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    fn file_name_error_str(e: crate::Error) -> String {
        match e {
            crate::Error::InvalidFileName(s) => s,
            _ => "Other error".to_string(),
        }
    }

    #[test]
    fn file_name_valid_filename() {
        let valid = ["file1.txt", "A1_b-2.C3", "abc.DEF", "Z-9_.z", "a1b2c3.d4"];
        for name in valid {
            let f = FileName::new(name);
            assert!(f.is_ok(), "Should accept valid filename: {}", name);
            assert_eq!(f.unwrap().as_str(), name);
        }
    }

    #[test]
    fn file_name_empty_filename() {
        let err = FileName::new("").unwrap_err();
        assert!(file_name_error_str(err).contains("empty"));
    }

    #[test]
    fn file_name_reserved_characters() {
        let reserved = [
            "file<.txt",
            "file>.txt",
            "file:.txt",
            "file\".txt",
            "file/.txt",
            "file\\.txt",
            "file|.txt",
            "file?.txt",
            "file*.txt",
        ];
        for name in reserved {
            let err = FileName::new(name).unwrap_err();
            assert!(file_name_error_str(err).contains("reserved"), "Failed for {}", name);
        }
    }

    #[test]
    fn file_name_invalid_characters() {
        let invalid = [
            "file@.txt",
            "file!.txt",
            "file$.txt",
            "file#.txt",
            "file%.txt",
            "file(.txt",
            "file).txt",
            "file,.txt",
            "file;.txt",
            "file=.txt",
            "file+.txt",
            "file~.txt",
            "file `.txt",
            "file .txt",
            "file\t.txt",
            "file\n.txt",
        ];
        for name in invalid {
            let err = FileName::new(name).unwrap_err();
            assert!(file_name_error_str(err).contains("invalid"), "Failed for {}", name);
        }
    }

    #[test]
    fn file_name_multiple_dots() {
        let names =
            ["file.name.txt", "a.b.c", "file..txt", ".hidden.file", "file.txt.", "filetxt.."];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(file_name_error_str(err).contains("exactly one '.'"), "Failed for {}", name);
        }
    }

    #[test]
    fn file_name_no_dot() {
        let names = ["filetxt", "abc", "file", "testfile"];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(file_name_error_str(err).contains("exactly one '.'"), "Failed for {}", name);
        }
    }

    #[test]
    fn file_name_empty_base_or_extension() {
        let names = [".txt", "file.", ".a", ".b", ".c", ".1", "1."];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(
                file_name_error_str(err).contains("non-empty base name and extension"),
                "Failed for {}",
                name
            );
        }
    }

    #[test]
    fn file_name_serialize_deserialize() {
        let name = "example1.txt";
        let fname = FileName::new(name).unwrap();
        let serialized = serde_json::to_string(&fname).unwrap();
        assert_eq!(serialized, format!("\"{}\"", name));
        let deserialized: FileName = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, fname);
    }

    #[test]
    fn cie_color_display_and_parse() {
        let cie = CieColor { x: 0.3127, y: 0.3290, yy: 1.0 };
        let s = cie.to_string();
        assert_eq!(s, "0.312700,0.329000,1.000000");

        let parsed: CieColor = s.parse().unwrap();
        assert!((parsed.x - 0.3127).abs() < 1e-6);
        assert!((parsed.y - 0.3290).abs() < 1e-6);
        assert!((parsed.yy - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cie_color_parse_invalid_format() {
        let bad_inputs = ["", "0.1,0.2", "0.1,0.2,0.3,0.4", "0.1;0.2;0.3", "0.1,0.2,", ",0.2,0.3"];
        for s in bad_inputs {
            let err = s.parse::<CieColor>().unwrap_err();
            match err {
                crate::Error::InvalidColorFormat(_) => {}
                _ => panic!("Unexpected error for input '{}'", s),
            }
        }
    }

    #[test]
    fn cie_color_serialize_deserialize() {
        let cie = CieColor { x: 0.123456, y: 0.654321, yy: 0.999999 };
        let serialized = serde_json::to_string(&cie).unwrap();
        assert_eq!(serialized, "\"0.123456,0.654321,0.999999\"");
        let deserialized: CieColor = serde_json::from_str(&serialized).unwrap();
        assert!((deserialized.x - cie.x).abs() < 1e-6);
        assert!((deserialized.y - cie.y).abs() < 1e-6);
        assert!((deserialized.yy - cie.yy).abs() < 1e-6);
    }
}
