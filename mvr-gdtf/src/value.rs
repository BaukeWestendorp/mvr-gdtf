use std::{
    fmt,
    str::{self, FromStr},
};

/// Represents a CIE 1931 xyY absolute color point.
///
/// See: MVR Spec Table 1: XML Generic Value Types, ValueType: CIE Color
/// See: GDTF Spec Table 1: XML Attribute Value Types, ValueType: ColorCIE
#[derive(Debug, Clone, PartialEq)]
pub struct CieColor {
    /// The x chromaticity coordinate.
    x: f32,
    /// The y chromaticity coordinate.
    y: f32,
    /// The Y luminance value.
    yy: f32,
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x_str = parts
            .next()
            .ok_or_else(|| crate::Error::InvalidCieColor { misformatted_color: s.to_owned() })?;
        let y_str = parts
            .next()
            .ok_or_else(|| crate::Error::InvalidCieColor { misformatted_color: s.to_owned() })?;
        let yy_str = parts
            .next()
            .ok_or_else(|| crate::Error::InvalidCieColor { misformatted_color: s.to_owned() })?;

        if parts.next().is_some() {
            return Err(crate::Error::InvalidCieColor { misformatted_color: s.to_owned() });
        }

        let x = x_str.trim().parse::<f32>().map_err(|e| crate::Error::CieColorParseXError {
            misformatted_string: format!("Failed to parse x component: {}", e),
        })?;
        let y = y_str.trim().parse::<f32>().map_err(|e| crate::Error::CieColorParseYError {
            misformatted_string: format!("Failed to parse y component: {}", e),
        })?;
        let yy = yy_str.trim().parse::<f32>().map_err(|e| crate::Error::CieColorParseYYError {
            misformatted_string: format!("Failed to parse Y component: {}", e),
        })?;

        Ok(CieColor { x, y, yy })
    }
}

impl fmt::Display for CieColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.yy)
    }
}

impl From<CieColor> for String {
    fn from(c: CieColor) -> Self {
        c.to_string()
    }
}

impl<'de> serde::Deserialize<'de> for CieColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        CieColor::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for CieColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
