use std::{
    fmt,
    str::{self, FromStr},
};

/// A case-sensitive name of a file within the MVR archive, including the extension.
///
/// See: MVR Spec Table 1: XML Generic Value Types, ValueType: FileName
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FileName(String);

impl FileName {
    /// Returns the inner string representation of the filename.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::FileName;
    /// # use std::str::FromStr;
    /// let fname = FileName::from_str("My-Fixture_5.gdtf").unwrap();
    /// assert_eq!(fname.as_str(), "My-Fixture_5.gdtf");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks if the given string is a valid FileName according to the rules.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::FileName;
    /// assert!(FileName::is_valid("My-Fixture_5.gdtf"));
    /// assert!(!FileName::is_valid("invalid/name.gdtf"));
    /// assert!(!FileName::is_valid(".hiddenfile"));
    /// assert!(!FileName::is_valid("noextension"));
    /// ```
    pub fn is_valid(s: &str) -> bool {
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

        if let (Some(ext), Some(base)) = (ext, base) {
            !base.is_empty() && !ext.is_empty()
        } else {
            false
        }
    }
}

impl str::FromStr for FileName {
    type Err = crate::Error;

    /// Parses a string into a [`FileName`].
    ///
    /// # Errors
    ///
    /// Returns [`InvalidFileName`] if the string violates MVR filename constraints.
    ///
    /// [`InvalidFileName`]: crate::Error::InvalidFileName
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if FileName::is_valid(s) {
            Ok(FileName(s.to_owned()))
        } else {
            Err(crate::Error::InvalidFileName(s.to_owned()))
        }
    }
}

impl TryFrom<String> for FileName {
    type Error = String;

    /// # Errors
    ///
    /// Returns a stringified error if the underlying [`FromStr`] validation fails.
    ///
    /// [`FromStr`]: std::str::FromStr
    fn try_from(value: String) -> Result<Self, Self::Error> {
        FileName::from_str(&value).map_err(|e| e.to_string())
    }
}

impl From<FileName> for String {
    fn from(f: FileName) -> Self {
        f.0
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Represents a CIE 1931 xyY absolute color point.
///
/// See: MVR Spec Table 1: XML Generic Value Types, ValueType: CIE Color
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CieColor {
    /// The x chromaticity coordinate.
    pub x: f32,
    /// The y chromaticity coordinate.
    pub y: f32,
    /// The Y luminance value.
    pub yy: f32,
}

impl str::FromStr for CieColor {
    type Err = crate::Error;

    /// Parses a comma-separated string into a [`CieColor`].
    ///
    /// # Errors
    ///
    /// Returns [`InvalidCieColor`] if there are not exactly 3 components.
    /// Returns parsing errors if the components cannot be parsed into `f32`.
    ///
    /// [`InvalidCieColor`]: crate::Error::InvalidCieColor
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x_str = parts.next().ok_or_else(|| crate::Error::InvalidCieColor(s.to_owned()))?;
        let y_str = parts.next().ok_or_else(|| crate::Error::InvalidCieColor(s.to_owned()))?;
        let yy_str = parts.next().ok_or_else(|| crate::Error::InvalidCieColor(s.to_owned()))?;

        if parts.next().is_some() {
            return Err(crate::Error::InvalidCieColor(s.to_owned()));
        }

        let x = x_str.trim().parse::<f32>().map_err(|e| {
            crate::Error::CieColorParseXError(format!("Failed to parse x component: {}", e))
        })?;
        let y = y_str.trim().parse::<f32>().map_err(|e| {
            crate::Error::CieColorParseYError(format!("Failed to parse y component: {}", e))
        })?;
        let yy = yy_str.trim().parse::<f32>().map_err(|e| {
            crate::Error::CieColorParseYYError(format!("Failed to parse Y component: {}", e))
        })?;

        Ok(CieColor { x, y, yy })
    }
}

impl fmt::Display for CieColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.yy)
    }
}

impl TryFrom<String> for CieColor {
    type Error = String;

    /// # Errors
    ///
    /// Returns a stringified error if the underlying [`FromStr`] parsing fails.
    ///
    /// [`FromStr`]: std::str::FromStr
    fn try_from(value: String) -> Result<Self, Self::Error> {
        CieColor::from_str(&value).map_err(|e| e.to_string())
    }
}

impl From<CieColor> for String {
    fn from(c: CieColor) -> Self {
        c.to_string()
    }
}

/// Represents a 4x3 transformation matrix for spatial transformations in MVR.
///
/// See: MVR Spec Table 35: Matrix Node Value Types
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Matrix {
    /// The first component of the u vector.
    pub u1: f32,
    /// The second component of the u vector.
    pub u2: f32,
    /// The third component of the u vector.
    pub u3: f32,
    /// The first component of the v vector.
    pub v1: f32,
    /// The second component of the v vector.
    pub v2: f32,
    /// The third component of the v vector.
    pub v3: f32,
    /// The first component of the w vector.
    pub w1: f32,
    /// The second component of the w vector.
    pub w2: f32,
    /// The third component of the w vector.
    pub w3: f32,
    /// The first component of the origin/offset vector.
    pub o1: f32,
    /// The second component of the origin/offset vector.
    pub o2: f32,
    /// The third component of the origin/offset vector.
    pub o3: f32,
}

impl str::FromStr for Matrix {
    type Err = crate::Error;

    /// Parses a bracketed string format into a [`Matrix`].
    ///
    /// # Errors
    ///
    /// Returns [`MatrixFormatError`] if the string does not contain exactly 12 values.
    /// Returns [`MatrixParseValueError`] if any value cannot be parsed as `f32`.
    ///
    /// [`MatrixFormatError`]: crate::Error::MatrixFormatError
    /// [`MatrixParseValueError`]: crate::Error::MatrixParseValueError
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let trimmed = s.trim_matches(|c| c == '{' || c == '}');

        let mut vals = Vec::with_capacity(12);
        for group in trimmed.split("}{") {
            for num in group.split(',') {
                let val = num.trim().parse::<f32>().map_err(|e| {
                    crate::Error::MatrixParseValueError(format!(
                        "Failed to parse matrix value: {}",
                        e
                    ))
                })?;
                vals.push(val);
            }
        }

        if vals.len() != 12 {
            return Err(crate::Error::MatrixFormatError(format!(
                "Expected 12 matrix values, got {}: '{}'",
                vals.len(),
                s
            )));
        }

        Ok(Matrix {
            u1: vals[0],
            u2: vals[1],
            u3: vals[2],
            v1: vals[3],
            v2: vals[4],
            v3: vals[5],
            w1: vals[6],
            w2: vals[7],
            w3: vals[8],
            o1: vals[9],
            o2: vals[10],
            o3: vals[11],
        })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}",
            self.u1,
            self.u2,
            self.u3,
            self.v1,
            self.v2,
            self.v3,
            self.w1,
            self.w2,
            self.w3,
            self.o1,
            self.o2,
            self.o3
        )
    }
}

impl TryFrom<String> for Matrix {
    type Error = String;

    /// # Errors
    ///
    /// Returns a stringified error if the underlying [`FromStr`] parsing fails.
    ///
    /// [`FromStr`]: std::str::FromStr
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Matrix::from_str(&value).map_err(|e| e.to_string())
    }
}

impl From<Matrix> for String {
    fn from(m: Matrix) -> Self {
        m.to_string()
    }
}
