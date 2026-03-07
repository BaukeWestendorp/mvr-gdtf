use std::{fmt, str};

use derive_more::{Debug, Display, FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmxValue {
    value: u32,
    bytes: u8,
    shifting: bool,
}

impl DmxValue {
    pub fn from_u8(value: u8, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 1, shifting }
    }

    pub fn from_u16(value: u16, shifting: bool) -> Self {
        DmxValue { value: value as u32, bytes: 2, shifting }
    }

    pub fn from_u24(value: u32, shifting: bool) -> Result<Self, crate::gdtf::Error> {
        if value > 0xFFFFFF {
            return Err(crate::gdtf::Error::DmxValueOutOfBounds { value, bytes: 3, max: 0xFFFFFF });
        }
        Ok(DmxValue { value, bytes: 3, shifting })
    }

    pub fn from_u32(value: u32, shifting: bool) -> Self {
        DmxValue { value, bytes: 4, shifting }
    }
}

impl Default for DmxValue {
    fn default() -> Self {
        Self { value: 0, bytes: 1, shifting: false }
    }
}

impl str::FromStr for DmxValue {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (value_s, rest) = s.split_once('/').ok_or_else(|| {
            crate::gdtf::Error::DmxValueParseError { misformatted_string: s.to_owned() }
        })?;

        let value: u32 = value_s.trim().parse().map_err(|_| {
            crate::gdtf::Error::DmxValueParseError { misformatted_string: s.to_owned() }
        })?;

        let rest = rest.trim();
        let (bytes_s, shifting) = if let Some(bytes_s) = rest.strip_suffix('s') {
            (bytes_s, true)
        } else {
            (rest, false)
        };

        let bytes_u32: u32 = bytes_s.parse().map_err(|_| {
            crate::gdtf::Error::DmxValueParseError { misformatted_string: s.to_owned() }
        })?;
        if bytes_u32 == 0 || bytes_u32 > u8::MAX as u32 {
            return Err(crate::gdtf::Error::DmxValueBytesZero { bytes: bytes_u32 as u8 });
        }

        Ok(DmxValue { value, bytes: bytes_u32 as u8, shifting })
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
        s.parse::<DmxValue>().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmxOffset {
    Offsets(Vec<u32>),
    Virtual,
}

impl fmt::Display for DmxOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DmxOffset::Virtual => Ok(()),
            DmxOffset::Offsets(values) => {
                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                Ok(())
            }
        }
    }
}

impl str::FromStr for DmxOffset {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Ok(DmxOffset::Virtual);
        }

        let mut values = Vec::new();
        for part in s.split(',') {
            if part.is_empty() {
                return Err(crate::gdtf::Error::DmxOffsetParseError {
                    misformatted_string: s.to_owned(),
                });
            }
            let v: u32 = part.trim().parse().map_err(|_| {
                crate::gdtf::Error::DmxOffsetParseError { misformatted_string: s.to_owned() }
            })?;
            values.push(v);
        }

        Ok(DmxOffset::Offsets(values))
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

/// Represents a DMX address in the range `1..=512`.
///
/// See: GDTF Spec Table 1: XML Attribute Value Types, ValueType: DMXAddress
///
/// DMX addresses are 1-based.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display("{}", _0)]
pub struct DmxAddress(u16);

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
    pub fn new(raw_address: u16) -> Result<Self, crate::gdtf::Error> {
        if (Self::MIN..=Self::MAX).contains(&raw_address) {
            Ok(Self(raw_address))
        } else {
            Err(crate::gdtf::Error::DmxAddressOutOfBounds { raw_address })
        }
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

impl TryFrom<u16> for DmxAddress {
    type Error = crate::gdtf::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        DmxAddress::new(value)
    }
}

impl str::FromStr for DmxAddress {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let value: u16 = s.parse().map_err(|_| crate::gdtf::Error::DmxAddressParseError {
            misformatted_string: s.to_string(),
        })?;
        DmxAddress::new(value)
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
        DmxAddress::new(v).map_err(serde::de::Error::custom)
    }
}

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum DmxBreak {
    /// Indicates the break should be overwritten.
    #[display("Overwrite")]
    Overwrite,
    /// A non-zero u16 representing the DMX break.
    #[display("{}", _0)]
    Number(u16),
}

impl Default for DmxBreak {
    fn default() -> Self {
        Self::Number(1)
    }
}

impl str::FromStr for DmxBreak {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "Overwrite" {
            Ok(DmxBreak::Overwrite)
        } else {
            match s.parse::<u32>() {
                Ok(n) if n > 0 => Ok(DmxBreak::Number(n as u16)),
                Ok(_) => Err(crate::gdtf::Error::DmxBreakZero),
                Err(_) => Err(crate::gdtf::Error::DmxBreakParseError {
                    misformatted_string: s.to_string(),
                }),
            }
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

/// Represents a 4x4 matrix for spatial transformations.
///
/// Format: "{a,b,c,d}{e,f,g,h}{i,j,k,l}{m,n,o,p}"
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix4x4 {
    /// The matrix data as a 2D array.
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
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

    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        &self.data
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
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('{') || !s.ends_with('}') {
            return Err(crate::gdtf::Error::MatrixFormatError {
                misformatted_string: s.to_string(),
            });
        }

        let mut blocks: Vec<&str> = Vec::with_capacity(4);
        let mut rest = s;
        while let Some(start) = rest.find('{') {
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('}') else {
                return Err(crate::gdtf::Error::MatrixFormatError {
                    misformatted_string: s.to_string(),
                });
            };
            blocks.push(&after_start[..end]);
            rest = &after_start[end + 1..];
        }

        if blocks.len() != 4 {
            return Err(crate::gdtf::Error::MatrixFormatError {
                misformatted_string: s.to_string(),
            });
        }

        let mut data = [[0.0f32; 4]; 4];
        for (i, block) in blocks.iter().enumerate() {
            let vals: Vec<&str> =
                block.split(',').map(str::trim).filter(|v| !v.is_empty()).collect();
            if vals.len() != 4 {
                return Err(crate::gdtf::Error::MatrixFormatError {
                    misformatted_string: s.to_string(),
                });
            }
            for (j, v) in vals.iter().enumerate() {
                data[i][j] = v.parse::<f32>().map_err(|_| {
                    crate::gdtf::Error::MatrixParseValueError { misformatted_string: v.to_string() }
                })?;
            }
        }

        Ok(Self { data })
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Display)]
#[display("{}", _0)]
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

impl str::FromStr for Name {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Name(s.to_owned()))
        } else {
            Err(crate::gdtf::Error::NameInvalid { misformatted_string: s.to_owned() })
        }
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

impl str::FromStr for Node {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(crate::gdtf::Error::NodeInvalid { misformatted_string: s.to_owned() });
        }

        let parts: Vec<&str> = s.split('.').collect();
        if parts.iter().any(|p| p.is_empty()) {
            return Err(crate::gdtf::Error::NodeInvalid { misformatted_string: s.to_owned() });
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

/// Represents a physical DMX percentage value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Display, FromStr)]
#[display("{}", _0)]
pub struct PhysicalValue(f32);

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
    pub fn new(value: f32) -> Result<Self, crate::gdtf::Error> {
        if value > 0.0 && value <= 100.0 {
            Ok(PhysicalValue(value))
        } else {
            Err(crate::gdtf::Error::PhysicalValueOutOfBounds { raw_value: value })
        }
    }

    /// Returns the inner value as `f32`.
    pub fn get(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for PhysicalValue {
    type Error = crate::gdtf::Error;

    /// Attempts to create a [`PhysicalValue`].
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not strictly greater than 0 and less than or equal to 100.
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        PhysicalValue::new(value)
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
        PhysicalValue::new(v).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl str::FromStr for Vector3 {
    type Err = crate::gdtf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Accept "{x,y,z}" or "x,y,z"
        let inner = if s.starts_with('{') && s.ends_with('}') { &s[1..s.len() - 1] } else { s };

        let mut parts = inner.split(',');
        let x_str = parts.next().ok_or_else(|| crate::gdtf::Error::Vector3ParseValueError {
            misformatted_string: s.to_owned(),
        })?;
        let y_str = parts.next().ok_or_else(|| crate::gdtf::Error::Vector3ParseValueError {
            misformatted_string: s.to_owned(),
        })?;
        let z_str = parts.next().ok_or_else(|| crate::gdtf::Error::Vector3ParseValueError {
            misformatted_string: s.to_owned(),
        })?;

        if parts.next().is_some() {
            return Err(crate::gdtf::Error::Vector3FormatError {
                misformatted_string: s.to_owned(),
            });
        }

        let x = x_str.trim().parse::<f32>().map_err(|_| {
            crate::gdtf::Error::Vector3ParseValueError {
                misformatted_string: x_str.trim().to_owned(),
            }
        })?;
        let y = y_str.trim().parse::<f32>().map_err(|_| {
            crate::gdtf::Error::Vector3ParseValueError {
                misformatted_string: y_str.trim().to_owned(),
            }
        })?;
        let z = z_str.trim().parse::<f32>().map_err(|_| {
            crate::gdtf::Error::Vector3ParseValueError {
                misformatted_string: z_str.trim().to_owned(),
            }
        })?;

        Ok(Vector3 { x, y, z })
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
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
        s.parse::<Vector3>().map_err(serde::de::Error::custom)
    }
}
