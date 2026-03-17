use std::{fmt, ops, str};

/// A case-sensitive name of a file within the MVR archive, including the extension.
///
/// See: MVR Spec Table 1: XML Generic Value Types, ValueType: FileName
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileName(String);

impl FileName {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(&self) -> bool {
        const RESERVED: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

        if self.0.is_empty()
            || self.0.contains(RESERVED)
            || self.0.chars().any(|c| c < '\u{20}')
            || self.0.starts_with(' ')
            || self.0.ends_with(' ')
            || self.0.starts_with('.')
            || self.0.ends_with('.')
        {
            return false;
        }

        let mut parts = self.0.rsplitn(2, '.');
        let ext = parts.next();
        let base = parts.next();

        match (base, ext) {
            (Some(base), Some(ext)) => !base.is_empty() && !ext.is_empty(),
            _ => false,
        }
    }
}

impl str::FromStr for FileName {
    type Err = crate::mvr::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_name = Self(s.to_owned());

        // FIXME: This check might be better off being moved into a validation function on a `MvrFile`.
        if !file_name.is_valid() {
            log::warn!("Parsed invalid/discouraged FileName: {s}");
        }

        Ok(file_name)
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
pub struct TransformMatrix {
    data: [[f32; 3]; 4],
}

impl TransformMatrix {
    pub fn identity() -> Self {
        Self { data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]] }
    }

    pub fn ux(&self) -> f32 {
        self.data[0][0]
    }

    pub fn uy(&self) -> f32 {
        self.data[0][1]
    }

    pub fn uz(&self) -> f32 {
        self.data[0][2]
    }

    pub fn vx(&self) -> f32 {
        self.data[1][0]
    }

    pub fn vy(&self) -> f32 {
        self.data[1][1]
    }

    pub fn vz(&self) -> f32 {
        self.data[1][2]
    }

    pub fn wx(&self) -> f32 {
        self.data[2][0]
    }

    pub fn wy(&self) -> f32 {
        self.data[2][1]
    }

    pub fn wz(&self) -> f32 {
        self.data[2][2]
    }

    pub fn ox(&self) -> f32 {
        self.data[3][0]
    }

    pub fn oy(&self) -> f32 {
        self.data[3][1]
    }

    pub fn oz(&self) -> f32 {
        self.data[3][2]
    }

    pub fn as_array(&self) -> &[[f32; 3]; 4] {
        &self.data
    }
}

impl Default for TransformMatrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl ops::Deref for TransformMatrix {
    type Target = [[f32; 3]; 4];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ops::DerefMut for TransformMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for TransformMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            write!(f, "{{{},{},{}}}", row[0], row[1], row[2])?;
        }
        Ok(())
    }
}

impl str::FromStr for TransformMatrix {
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

impl<'de> serde::Deserialize<'de> for TransformMatrix {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(|e| serde::de::Error::custom(e))
    }
}
