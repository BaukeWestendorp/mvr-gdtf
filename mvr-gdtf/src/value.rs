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
    luminance: f32,
}
impl CieColor {
    #[inline]
    pub const fn new() -> Self {
        Self::white()
    }

    #[inline]
    pub const fn from_xy_lum(x: f32, y: f32, luminance: f32) -> Self {
        Self { x, y, luminance }
    }

    #[inline]
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        // CREDIT(Copyright MVR Group): https://github.com/mvrdevelopment/libMVRgdtf/blob/0b5fbaf2c04f2e4bcc0bc9929fa0097489cac72a/src/CieColor.cpp#L31

        // Convert RGB to CIE_xyz (range [0..1])
        let x_tmp = 0.412424 * r + 0.357579 * g + 0.180464 * b;
        let y_tmp = 0.212656 * r + 0.715158 * g + 0.0721856 * b;
        let z_tmp = 0.0193324 * r + 0.119193 * g + 0.950444 * b;

        // Convert CIE_xyz to CIE_xyY
        let (x, y, luminance) =
            if (x_tmp.abs() < 0.000001) || (y_tmp.abs() < 0.000001) || (z_tmp.abs() < 0.000001) {
                (0.0, 0.0, 0.0)
            } else {
                let sum = x_tmp + y_tmp + z_tmp;
                let x = x_tmp / sum;
                let y = y_tmp / sum;
                let luminance = y_tmp * 100.0; // Scale Y_luminance by 100
                (x, y, luminance)
            };

        Self::from_xy_lum(x, y, luminance)
    }

    #[inline]
    pub const fn white() -> Self {
        Self { x: 0.31273, y: 0.32902, luminance: 100.0 }
    }

    #[inline]
    pub const fn red() -> Self {
        Self::from_rgb(1.0, 0.0, 0.0)
    }

    #[inline]
    pub const fn green() -> Self {
        Self::from_rgb(0.0, 1.0, 0.0)
    }

    #[inline]
    pub const fn blue() -> Self {
        Self::from_rgb(0.0, 0.0, 1.0)
    }

    #[inline]
    pub const fn cyan() -> Self {
        Self::from_rgb(0.0, 1.0, 1.0)
    }

    #[inline]
    pub const fn magenta() -> Self {
        Self::from_rgb(1.0, 0.0, 1.0)
    }

    #[inline]
    pub const fn yellow() -> Self {
        Self::from_rgb(1.0, 1.0, 0.0)
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    pub const fn luminance(&self) -> f32 {
        self.luminance
    }

    pub fn to_rgb(&self) -> (f32, f32, f32) {
        // CREDIT(Copyright MVR Group): https://github.com/mvrdevelopment/libMVRgdtf/blob/0b5fbaf2c04f2e4bcc0bc9929fa0097489cac72a/src/CieColor.cpp#L83

        // Conversion formulas/matrix from http://www.brucelindbloom.com/
        let x = self.x;
        let y = self.y;
        let y_luminance = self.luminance;

        let (x_tmp, y_tmp, z_tmp) = if y.abs() < 0.000001 {
            (0.0, 0.0, 0.0)
        } else {
            let y_luminance_scaled = y_luminance / 100.0; // Scale Y_luminance by 100
            let x_tmp = (x * y_luminance_scaled) / y;
            let y_tmp = y_luminance_scaled;
            let z_tmp = ((1.0 - x - y) * y_luminance_scaled) / y;
            (x_tmp, y_tmp, z_tmp)
        };

        // Convert CIE_XYZ to linear RGB (values [0..1])
        let mut r = x_tmp * 3.24071 + y_tmp * (-1.53726) + z_tmp * (-0.498571);
        let mut g = x_tmp * (-0.969258) + y_tmp * 1.87599 + z_tmp * 0.0415557;
        let mut b = x_tmp * 0.0556352 + y_tmp * (-0.203996) + z_tmp * 1.05707;

        // Apply gamma correction to convert linear RGB to sRGB
        #[inline]
        fn gamma_correct(val: f32) -> f32 {
            if val > 0.0031308 { 1.055 * val.powf(1.0 / 2.4) - 0.055 } else { 12.92 * val }
        }

        r = gamma_correct(r);
        g = gamma_correct(g);
        b = gamma_correct(b);

        // Clamp between 0 and 1
        r = r.max(0.0).min(1.0);
        g = g.max(0.0).min(1.0);
        b = b.max(0.0).min(1.0);

        // Convert linear RGB [0..1] to sRGB [0..255]
        r *= 255.0;
        g *= 255.0;
        b *= 255.0;

        (r, g, b)
    }
}

impl Default for CieColor {
    fn default() -> Self {
        Self::new()
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
        let luminance_str = parts
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
        let luminance = luminance_str.trim().parse::<f32>().map_err(|e| {
            crate::Error::CieColorParseYYError {
                misformatted_string: format!("Failed to parse Y component: {}", e),
            }
        })?;

        Ok(CieColor { x, y, luminance })
    }
}

impl fmt::Display for CieColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.luminance)
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
