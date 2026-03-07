use std::{fmt, str};

/// A case-sensitive name of a file within the MVR archive, including the extension.
///
/// See: MVR Spec Table 1: XML Generic Value Types, ValueType: FileName
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileName(String);

impl FileName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl str::FromStr for FileName {
    type Err = crate::mvr::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn is_valid(s: &str) -> bool {
            const RESERVED: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

            if s.is_empty()
                || s.contains(RESERVED)
                || s.chars().any(|c| c < '\u{20}')
                || s.starts_with(' ')
                || s.ends_with(' ')
                || s.starts_with('.')
                || s.ends_with('.')
            {
                return false;
            }

            let mut parts = s.rsplitn(2, '.');
            let ext = parts.next();
            let base = parts.next();

            match (base, ext) {
                (Some(base), Some(ext)) => !base.is_empty() && !ext.is_empty(),
                _ => false,
            }
        }

        if is_valid(s) {
            Ok(Self(s.to_owned()))
        } else {
            Err(crate::mvr::Error::InvalidFileName { misformatted_name: s.to_string() })
        }
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::Deserialize<'de> for FileName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(|e| serde::de::Error::custom(e))
    }
}

/// Represents a 4x3 matrix for spatial transformations.
///
/// Format: `"{a,b,c}{d,e,f}{g,h,i}{j,k,l}"`.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x3 {
    /// The matrix data as a 2D array.
    pub data: [[f32; 3]; 4],
}

impl Matrix4x3 {
    /// Returns the matrix as an array.
    pub fn as_array(&self) -> &[[f32; 3]; 4] {
        &self.data
    }

    pub fn identity() -> Self {
        Self { data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]] }
    }

    /// Returns the matrix as a [`Vec<Vec<f32>>`].
    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.data.iter().map(|row| row.to_vec()).collect()
    }
}

impl Default for Matrix4x3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Matrix4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            write!(f, "{{{},{},{}}}", row[0], row[1], row[2])?;
        }
        Ok(())
    }
}

impl str::FromStr for Matrix4x3 {
    type Err = crate::mvr::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Expect exactly 4 blocks of "{a,b,c}" with no other content.
        let mut blocks: Vec<&str> = Vec::with_capacity(4);
        let mut rest = s;

        while let Some(start) = rest.find('{') {
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('}') else {
                return Err(crate::mvr::Error::MatrixFormatError {
                    misformatted_string: s.to_owned(),
                });
            };

            blocks.push(after_start[..end].trim());
            rest = &after_start[end + 1..];
        }

        if !rest.trim().is_empty() || blocks.len() != 4 {
            return Err(crate::mvr::Error::MatrixFormatError { misformatted_string: s.to_owned() });
        }

        let mut data = [[0.0f32; 3]; 4];
        for (i, block) in blocks.iter().enumerate() {
            let vals: Vec<&str> = block.split(',').map(str::trim).collect();
            if vals.len() != 3 || vals.iter().any(|v| v.is_empty()) {
                return Err(crate::mvr::Error::MatrixFormatError {
                    misformatted_string: s.to_owned(),
                });
            }

            for (j, v) in vals.iter().enumerate() {
                data[i][j] = v.parse::<f32>().map_err(|_| {
                    crate::mvr::Error::MatrixParseValueError { misformatted_string: s.to_owned() }
                })?;
            }
        }

        Ok(Self { data })
    }
}

impl<'de> serde::Deserialize<'de> for Matrix4x3 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(|e| serde::de::Error::custom(e))
    }
}
