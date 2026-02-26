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

#[cfg(test)]
mod tests {
    use super::*;

    fn error_str(e: crate::Error) -> String {
        match e {
            crate::Error::InvalidFileName(s) => s,
            _ => "Other error".to_string(),
        }
    }

    #[test]
    fn valid_filename() {
        let valid = ["file1.txt", "A1_b-2.C3", "abc.DEF", "Z-9_.z", "a1b2c3.d4"];
        for name in valid {
            let f = FileName::new(name);
            assert!(f.is_ok(), "Should accept valid filename: {}", name);
            assert_eq!(f.unwrap().as_str(), name);
        }
    }

    #[test]
    fn empty_filename() {
        let err = FileName::new("").unwrap_err();
        assert!(error_str(err).contains("empty"));
    }

    #[test]
    fn reserved_characters() {
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
            assert!(error_str(err).contains("reserved"), "Failed for {}", name);
        }
    }

    #[test]
    fn invalid_characters() {
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
            assert!(error_str(err).contains("invalid"), "Failed for {}", name);
        }
    }

    #[test]
    fn multiple_dots() {
        let names = [
            "file.name.txt",
            "a.b.c",
            "file..txt",
            ".hidden.file",
            "file.txt.",
            "filetxt..",
        ];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(
                error_str(err).contains("exactly one '.'"),
                "Failed for {}",
                name
            );
        }
    }

    #[test]
    fn no_dot() {
        let names = ["filetxt", "abc", "file", "testfile"];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(
                error_str(err).contains("exactly one '.'"),
                "Failed for {}",
                name
            );
        }
    }

    #[test]
    fn empty_base_or_extension() {
        let names = [".txt", "file.", ".a", ".b", ".c", ".1", "1."];
        for name in names {
            let err = FileName::new(name).unwrap_err();
            assert!(
                error_str(err).contains("non-empty base name and extension"),
                "Failed for {}",
                name
            );
        }
    }
}
