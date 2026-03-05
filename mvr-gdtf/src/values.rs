use std::{
    fmt,
    str::{self, FromStr},
};

/// Represents a DMX Break Type as defined in the GDTF specification.
///
/// See: GDTF Spec Table 1: XML Attribute Value Types, ValueType: DMXBreak
///
/// The DMX Break Type is either the string `"Overwrite"` or a non-zero `u16` value.
/// This is used to identify a DMX break or indicate that the break should be overwritten.
///
/// # Examples
///
/// ```
/// # use mvr_gdtf::DmxBreak;
/// assert!("Overwrite".parse::<DmxBreak>().is_ok());
/// assert!("1".parse::<DmxBreak>().is_ok());
/// assert!("512".parse::<DmxBreak>().is_ok());
/// assert!("".parse::<DmxBreak>().is_err());
/// assert!("abc".parse::<DmxBreak>().is_err());
/// assert!("0".parse::<DmxBreak>().is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DmxBreak {
    /// Indicates the break should be overwritten.
    Overwrite,
    /// A non-zero u16 representing the DMX break.
    Number(u16),
}

impl Default for DmxBreak {
    fn default() -> Self {
        Self::Number(1)
    }
}

impl str::FromStr for DmxBreak {
    type Err = crate::Error;

    /// Parses a string into a [`DmxBreak`] according to the GDTF specification.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Overwrite" {
            Ok(DmxBreak::Overwrite)
        } else {
            match s.parse::<u32>() {
                Ok(n) if n > 0 => Ok(DmxBreak::Number(n as u16)),
                Ok(_) => Err(crate::Error::DmxBreakZero),
                Err(e) => Err(crate::Error::DmxBreakParseError(format!(
                    "Failed to parse DMXBreak number: {}",
                    e
                ))),
            }
        }
    }
}

impl fmt::Display for DmxBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DmxBreak::Overwrite => write!(f, "Overwrite"),
            DmxBreak::Number(n) => write!(f, "{}", n),
        }
    }
}

impl serde::Serialize for DmxBreak {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DmxBreak {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<DmxBreak>().map_err(|e| serde::de::Error::custom(e))
    }
}

/// Represents a DMX address in the range 1..=512.
///
/// See: GDTF Spec Table 1: XML Attribute Value Types, ValueType: DMXAddress
///
/// DMX addresses are 1-based and must be in the inclusive range 1..=512.
///
/// # Examples
///
/// ```
/// # use mvr_gdtf::DmxAddress;
/// assert!(DmxAddress::new(1).is_some());
/// assert!(DmxAddress::new(512).is_some());
/// assert!(DmxAddress::new(0).is_none());
/// assert!(DmxAddress::new(513).is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DmxAddress(pub u16);

impl DmxAddress {
    /// The minimum valid DMX address (inclusive).
    pub const MIN: u16 = 1;
    /// The maximum valid DMX address (inclusive).
    pub const MAX: u16 = 512;

    /// Creates a new [`DmxAddress`] if the value is in the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mvr_gdtf::DmxAddress;
    /// assert!(DmxAddress::new(1).is_some());
    /// assert!(DmxAddress::new(512).is_some());
    /// assert!(DmxAddress::new(0).is_none());
    /// assert!(DmxAddress::new(513).is_none());
    /// ```
    pub fn new(value: u16) -> Option<Self> {
        if (Self::MIN..=Self::MAX).contains(&value) { Some(Self(value)) } else { None }
    }

    /// Returns the inner DMX address value as `u16`.
    pub fn get(self) -> u16 {
        self.0
    }
}

impl Default for DmxAddress {
    fn default() -> Self {
        Self(Self::MIN)
    }
}

impl fmt::Display for DmxAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<u16> for DmxAddress {
    type Error = crate::Error;

    /// Attempts to create a [`DmxAddress`] from a `u16`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not in the range 1..=512.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        DmxAddress::new(value).ok_or(crate::Error::InvalidDmxAddress(format!(
            "Invalid DMX address: {} (must be in 1..=512)",
            value
        )))
    }
}

impl str::FromStr for DmxAddress {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let value: u16 = s.parse().map_err(|e| {
            crate::Error::InvalidDmxAddress(format!(
                "Failed to parse DMXAddress from '{}': {}",
                s, e
            ))
        })?;
        DmxAddress::new(value).ok_or(crate::Error::InvalidDmxAddress(format!(
            "Invalid DMX address: {} (must be in 1..=512)",
            value
        )))
    }
}

impl serde::Serialize for DmxAddress {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(self.0)
    }
}

impl<'de> serde::Deserialize<'de> for DmxAddress {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = u16::deserialize(deserializer)?;
        DmxAddress::new(v).ok_or_else(|| {
            serde::de::Error::custom(crate::Error::InvalidDmxAddress(format!(
                "Invalid DMX address: {} (must be in 1..=512)",
                v
            )))
        })
    }
}

/// Represents a physical DMX percentage value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhysicalValue(pub f32);

impl PhysicalValue {
    /// Creates a new [`PhysicalValue`].
    ///
    /// # Example
    ///
    /// ```
    /// # use mvr_gdtf::PhysicalValue;
    /// assert!(PhysicalValue::new(50.0).is_some());
    /// assert!(PhysicalValue::new(0.0).is_none());
    /// assert!(PhysicalValue::new(101.0).is_none());
    /// ```
    pub fn new(value: f32) -> Option<Self> {
        if value > 0.0 && value <= 100.0 { Some(PhysicalValue(value)) } else { None }
    }

    /// Returns the inner value as `f32`.
    pub fn get(self) -> f32 {
        self.0
    }
}

impl fmt::Display for PhysicalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<f32> for PhysicalValue {
    type Error = crate::Error;

    /// Attempts to create a [`PhysicalValue`].
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not strictly greater than 0 and less than or equal to 100.
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        PhysicalValue::new(value).ok_or(crate::Error::InvalidPhysicalValue(value))
    }
}

impl serde::Serialize for PhysicalValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f32(self.0)
    }
}

impl<'de> serde::Deserialize<'de> for PhysicalValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = f32::deserialize(deserializer)?;
        PhysicalValue::new(v)
            .ok_or_else(|| serde::de::Error::custom(crate::Error::InvalidPhysicalValue(v)))
    }
}

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
    /// # use mvr_gdtf::FileName;
    /// # use std::str::FromStr;
    /// let fname = FileName::from_str("Fixture_5.gdtf").unwrap();
    /// assert_eq!(fname.as_str(), "Fixture_5.gdtf");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Checks if the given string is a valid FileName according to the rules.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mvr_gdtf::FileName;
    /// assert!(FileName::is_valid("Fixture_5.gdtf"));
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
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CieColor {
    /// The x chromaticity coordinate.
    pub x: f32,
    /// The y chromaticity coordinate.
    pub y: f32,
    /// The Y luminance value.
    pub yy: f32,
}

impl CieColor {
    pub fn white() -> Self {
        Self { x: 0.3127, y: 0.3290, yy: 100.0 }
    }
}

impl Default for CieColor {
    fn default() -> Self {
        Self::white()
    }
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
/// Represents a 4x4 matrix for spatial transformations.
///
/// Format: "{a,b,c,d}{e,f,g,h}{i,j,k,l}{m,n,o,p}"
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4 {
    /// The matrix data as a 2D array.
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    /// Returns the matrix as an array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mvr_gdtf::Matrix4x4;
    /// let m = Matrix4x4::from_str("{1,2,3,4}{5,6,7,8}{9,10,11,12}{13,14,15,16}").unwrap();
    /// assert_eq!(m.data, [
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0],
    ///     [13.0, 14.0, 15.0, 16.0]
    /// ]);
    /// ```
    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        &self.data
    }

    /// Creates a new Matrix4x4 if the string is valid.
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, crate::Error> {
        let s = s.as_ref().trim();

        if !s.starts_with('{') || !s.ends_with('}') {
            return Err(crate::Error::MatrixFormatError("Invalid Matrix4x4 format".to_owned()));
        }

        let mut blocks: Vec<&str> = Vec::with_capacity(4);
        let mut rest = s;
        while let Some(start) = rest.find('{') {
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('}') else {
                return Err(crate::Error::MatrixFormatError("Invalid Matrix4x4 format".to_owned()));
            };
            blocks.push(&after_start[..end]);
            rest = &after_start[end + 1..];
        }

        if blocks.len() != 4 {
            return Err(crate::Error::MatrixFormatError("Invalid Matrix4x4 format".to_owned()));
        }

        let mut data = [[0.0f32; 4]; 4];
        for (i, block) in blocks.iter().enumerate() {
            let vals: Vec<&str> =
                block.split(',').map(str::trim).filter(|v| !v.is_empty()).collect();
            if vals.len() != 4 {
                return Err(crate::Error::MatrixFormatError("Invalid Matrix4x4 format".to_owned()));
            }
            for (j, v) in vals.iter().enumerate() {
                data[i][j] = v.parse::<f32>().map_err(|e| {
                    crate::Error::MatrixParseValueError(format!(
                        "Failed to parse Matrix4x4 value as f32: {}",
                        e
                    ))
                })?;
            }
        }

        Ok(Matrix4x4 { data })
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<f32>> {
        self.data.iter().map(|row| row.to_vec()).collect()
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in self.data.iter() {
            s.push('{');
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&val.to_string());
            }
            s.push('}');
        }
        write!(f, "{}", s)
    }
}

impl str::FromStr for Matrix4x4 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for Matrix4x4 {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<Matrix4x4>().map_err(|e| e.to_string())
    }
}

impl serde::Serialize for Matrix4x4 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Matrix4x4 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Matrix4x4>().map_err(|e| serde::de::Error::custom(e))
    }
}

/// Represents a 4x3 matrix for spatial transformations.
///
/// Format: "{a,b,c}{d,e,f}{g,h,i}{j,k,l}"
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x3 {
    /// The matrix data as a 2D array.
    pub data: [[f32; 3]; 4],
}

impl Matrix4x3 {
    /// Returns the matrix as an array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mvr_gdtf::Matrix4x3;
    /// let m = Matrix4x3::from_str("{1,2,3}{4,5,6}{7,8,9}{10,11,12}").unwrap();
    /// assert_eq!(m.data, [
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 9.0],
    ///     [10.0, 11.0, 12.0]
    /// ]);
    /// ```
    pub fn as_array(&self) -> &[[f32; 3]; 4] {
        &self.data
    }

    /// Creates a new [`Matrix4x3`] if the string is valid.
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, crate::Error> {
        let s = s.as_ref().trim();

        if !s.starts_with('{') || !s.ends_with('}') {
            return Err(crate::Error::MatrixFormatError("Invalid Matrix4x3 format".to_owned()));
        }

        let mut blocks: Vec<&str> = Vec::with_capacity(4);
        let mut rest = s;
        while let Some(start) = rest.find('{') {
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('}') else {
                return Err(crate::Error::MatrixFormatError("Invalid Matrix4x3 format".to_owned()));
            };
            blocks.push(&after_start[..end]);
            rest = &after_start[end + 1..];
        }

        if blocks.len() != 4 {
            return Err(crate::Error::MatrixFormatError("Invalid Matrix4x3 format".to_owned()));
        }

        let mut data = [[0.0f32; 3]; 4];
        for (i, block) in blocks.iter().enumerate() {
            let vals: Vec<&str> =
                block.split(',').map(str::trim).filter(|v| !v.is_empty()).collect();
            if vals.len() != 3 {
                return Err(crate::Error::MatrixFormatError("Invalid Matrix4x3 format".to_owned()));
            }
            for (j, v) in vals.iter().enumerate() {
                data[i][j] = v.parse::<f32>().map_err(|e| {
                    crate::Error::MatrixParseValueError(format!(
                        "Failed to parse Matrix4x3 value as f32: {}",
                        e
                    ))
                })?;
            }
        }

        Ok(Matrix4x3 { data })
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
        let mut s = String::new();
        for row in self.data.iter() {
            s.push('{');
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    s.push(',');
                }
                s.push_str(&val.to_string());
            }
            s.push('}');
        }
        write!(f, "{}", s)
    }
}

impl str::FromStr for Matrix4x3 {
    type Err = crate::Error;

    /// Parses a string into a [`Matrix4x3`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for Matrix4x3 {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<Matrix4x3>().map_err(|e| e.to_string())
    }
}

impl serde::Serialize for Matrix4x3 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Matrix4x3 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Matrix4x3>().map_err(|e| serde::de::Error::custom(e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Name(String);

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_valid(s: &str) -> bool {
        if s.is_empty() {
            return true;
        }

        s.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(
                    c,
                    '#' | '%'
                        | '('
                        | ')'
                        | '*'
                        | '+'
                        | '-'
                        | '/'
                        | ':'
                        | ';'
                        | '<'
                        | '='
                        | '>'
                        | '@'
                        | '_'
                        | '`'
                        | ' '
                        | '"'
                        | '\''
                )
        })
    }
}

impl FromStr for Name {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Name(s.to_owned()))
        } else {
            Err(crate::Error::InvalidName(s.to_owned()))
        }
    }
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<Name>().map_err(|e| e.to_string())
    }
}

impl From<Name> for String {
    fn from(n: Name) -> Self {
        n.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::Serialize for Name {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Name>().map_err(serde::de::Error::custom)
    }
}

/// A dot-separated node reference, like `Name.Name.Name`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(Vec<Name>);

impl Node {
    pub fn segments(&self) -> &[Name] {
        &self.0
    }
}

impl FromStr for Node {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(crate::Error::InvalidNode(s.to_owned()));
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.iter().any(|p| p.is_empty()) {
            return Err(crate::Error::InvalidNode(s.to_owned()));
        }

        let mut names = Vec::with_capacity(parts.len());
        for p in parts {
            names.push(p.parse::<Name>()?);
        }

        Ok(Node(names))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{seg}")?;
        }
        Ok(())
    }
}

impl serde::Serialize for Node {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Node>().map_err(serde::de::Error::custom)
    }
}

/// A feature identifier `Group.Feature` (exactly 2 `Name` segments).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureType {
    pub group: Name,
    pub feature: Name,
}

impl FromStr for FeatureType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (a, b) =
            s.split_once('.').ok_or_else(|| crate::Error::InvalidFeatureType(s.to_owned()))?;

        if a.is_empty() || b.is_empty() {
            return Err(crate::Error::InvalidFeatureType(s.to_owned()));
        }

        let group: Name = a.parse()?;
        let feature: Name = b.parse()?;
        Ok(FeatureType { group, feature })
    }
}

impl fmt::Display for FeatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.group, self.feature)
    }
}

impl serde::Serialize for FeatureType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for FeatureType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<FeatureType>().map_err(serde::de::Error::custom)
    }
}

/// A 3-component vector formatted as `x,y,z`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3(pub [f32; 3]);

impl Vector3 {
    pub fn as_array(&self) -> [f32; 3] {
        self.0
    }
}

impl FromStr for Vector3 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s0 = s;
        let s = s.trim();

        let mut parts = s.split(',');
        let x_str = parts.next().ok_or_else(|| crate::Error::InvalidVector3(s0.to_owned()))?;
        let y_str = parts.next().ok_or_else(|| crate::Error::InvalidVector3(s0.to_owned()))?;
        let z_str = parts.next().ok_or_else(|| crate::Error::InvalidVector3(s0.to_owned()))?;

        if parts.next().is_some() {
            return Err(crate::Error::InvalidVector3(s0.to_owned()));
        }

        let x =
            x_str.trim().parse::<f32>().map_err(|_| crate::Error::InvalidVector3(s0.to_owned()))?;
        let y =
            y_str.trim().parse::<f32>().map_err(|_| crate::Error::InvalidVector3(s0.to_owned()))?;
        let z =
            z_str.trim().parse::<f32>().map_err(|_| crate::Error::InvalidVector3(s0.to_owned()))?;

        Ok(Vector3([x, y, z]))
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.0[0], self.0[1], self.0[2])
    }
}

impl serde::Serialize for Vector3 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Vector3 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.trim() == "None" {
            return Err(serde::de::Error::custom(crate::Error::InvalidVector3("None".to_owned())));
        }
        s.parse::<Vector3>().map_err(serde::de::Error::custom)
    }
}

/// An offset list formatted as `n[,n]*` or empty string for Virtual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmxOffset {
    Virtual,
    Offsets(Vec<u32>),
}

impl FromStr for DmxOffset {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s0 = s;
        let s = s.trim();

        if s.is_empty() {
            return Ok(DmxOffset::Virtual);
        }

        let mut values = Vec::new();
        for part in s.split(',') {
            if part.is_empty() {
                return Err(crate::Error::InvalidOffset(s0.to_owned()));
            }
            let v: u32 =
                part.trim().parse().map_err(|_| crate::Error::InvalidOffset(s0.to_owned()))?;
            values.push(v);
        }

        Ok(DmxOffset::Offsets(values))
    }
}

impl fmt::Display for DmxOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DmxOffset::Virtual => write!(f, ""),
            DmxOffset::Offsets(values) => {
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{v}")?;
                }
                Ok(())
            }
        }
    }
}

impl serde::Serialize for DmxOffset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DmxOffset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<DmxOffset>().map_err(serde::de::Error::custom)
    }
}

/// A DMX value specification formatted as:
/// - `Uint/n`
/// - `Uint/ns`
///
/// Where `n` is the byte count (>= 1), and optional trailing `s` indicates byte-shifting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmxValue {
    pub value: u32,
    pub bytes: u8,
    pub shifting: bool,
}

impl DmxValue {
    pub fn new(value: u32, bytes: u8, shifting: bool) -> Result<Self, crate::Error> {
        if bytes == 0 {
            return Err(crate::Error::InvalidDmxValue(format!("bytes must be >= 1 (got {bytes})")));
        }

        let max = if bytes >= 4 { u32::MAX } else { (1u32 << (8 * (bytes as u32))) - 1 };
        if value > max {
            return Err(crate::Error::InvalidDmxValue(format!(
                "value {value} does not fit in {bytes} bytes (max {max})"
            )));
        }

        Ok(DmxValue { value, bytes, shifting })
    }
}

impl Default for DmxValue {
    fn default() -> Self {
        Self { value: 0, bytes: 1, shifting: false }
    }
}

impl FromStr for DmxValue {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s0 = s;
        let s = s.trim();

        let (value_s, rest) =
            s.split_once('/').ok_or_else(|| crate::Error::InvalidDmxValue(s0.to_owned()))?;

        let value: u32 =
            value_s.trim().parse().map_err(|_| crate::Error::InvalidDmxValue(s0.to_owned()))?;

        let rest = rest.trim();
        let (bytes_s, shifting) = if let Some(bytes_s) = rest.strip_suffix('s') {
            (bytes_s, true)
        } else {
            (rest, false)
        };

        let bytes_u32: u32 =
            bytes_s.parse().map_err(|_| crate::Error::InvalidDmxValue(s0.to_owned()))?;
        if bytes_u32 == 0 || bytes_u32 > u8::MAX as u32 {
            return Err(crate::Error::InvalidDmxValue(s0.to_owned()));
        }

        DmxValue::new(value, bytes_u32 as u8, shifting)
    }
}

impl fmt::Display for DmxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shifting {
            write!(f, "{}/{}s", self.value, self.bytes)
        } else {
            write!(f, "{}/{}", self.value, self.bytes)
        }
    }
}

impl serde::Serialize for DmxValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DmxValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.trim() == "None" {
            return Err(serde::de::Error::custom(crate::Error::InvalidDmxValue("None".to_owned())));
        }
        s.parse::<DmxValue>().map_err(serde::de::Error::custom)
    }
}
