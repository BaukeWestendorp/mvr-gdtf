//! Values that are both used in MVR and GDTF.

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
        let mut x_tmp = 0.412424 * r + 0.357579 * g + 0.180464 * b;
        let mut y_tmp = 0.212656 * r + 0.715158 * g + 0.0721856 * b;
        let mut z_tmp = 0.0193324 * r + 0.119193 * g + 0.950444 * b;

        if x_tmp > 0.99999 {
            x_tmp = 1.0;
        }
        if y_tmp > 0.99999 {
            y_tmp = 1.0;
        }
        if z_tmp > 0.99999 {
            z_tmp = 1.0;
        }

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
        Self::from_rgb(1.0, 1.0, 1.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(label: &str, a: f32, b: f32, eps: f32) {
        let diff = (a - b).abs();
        assert!(diff <= eps, "{label}: {a} vs {b} (diff={diff}, eps={eps})");
    }

    fn assert_rgb_in_0_1(label: &str, (r, g, b): (f32, f32, f32)) {
        assert!((0.0..=1.0).contains(&r), "{label}: r out of range: {r}");
        assert!((0.0..=1.0).contains(&g), "{label}: g out of range: {g}");
        assert!((0.0..=1.0).contains(&b), "{label}: b out of range: {b}");
    }

    #[test]
    fn parse_valid_cie_color_strict_three_components_with_whitespace() {
        let c: CieColor = " 0.3127 , 0.3290 , 100 ".parse().unwrap();
        assert_approx_eq("x", c.x(), 0.3127, 1e-6);
        assert_approx_eq("y", c.y(), 0.3290, 1e-6);
        assert_approx_eq("Y", c.luminance(), 100.0, 1e-6);
    }

    #[test]
    fn parse_rejects_too_few_components() {
        let err = "0.3127,0.3290".parse::<CieColor>().unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.to_lowercase().contains("cie") || msg.to_lowercase().contains("color"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn parse_rejects_too_many_components() {
        let err = "0.3127,0.3290,100,1.0".parse::<CieColor>().unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.to_lowercase().contains("cie") || msg.to_lowercase().contains("color"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn parse_rejects_non_numeric_components_with_specific_error_context() {
        let err = "x,0.329,100".parse::<CieColor>().unwrap_err();
        let s = err.to_string().to_lowercase();
        assert!(
            s.contains("parse") && (s.contains("x") || s.contains("component")),
            "unexpected error message: {s}"
        );

        let err = "0.3127,y,100".parse::<CieColor>().unwrap_err();
        let s = err.to_string().to_lowercase();
        assert!(
            s.contains("parse") && (s.contains("y") || s.contains("component")),
            "unexpected error message: {s}"
        );

        let err = "0.3127,0.3290,yy".parse::<CieColor>().unwrap_err();
        let s = err.to_string().to_lowercase();
        assert!(
            s.contains("parse") && (s.contains("y") || s.contains("component")),
            "unexpected error message: {s}"
        );
    }

    #[test]
    fn display_round_trip_parse_is_stable_enough() {
        let original = CieColor::from_xy_lum(0.4, 0.5, 12.34);
        let s = original.to_string();
        let parsed: CieColor = s.parse().unwrap();

        assert_approx_eq("x", parsed.x(), original.x(), 1e-6);
        assert_approx_eq("y", parsed.y(), original.y(), 1e-6);
        assert_approx_eq("Y", parsed.luminance(), original.luminance(), 1e-6);
    }

    #[test]
    fn rgb_to_cie_primary_points_are_plausible_and_not_nan() {
        for (name, c) in [
            ("white", CieColor::white()),
            ("red", CieColor::red()),
            ("green", CieColor::green()),
            ("blue", CieColor::blue()),
            ("cyan", CieColor::cyan()),
            ("magenta", CieColor::magenta()),
            ("yellow", CieColor::yellow()),
        ] {
            assert!(!c.x().is_nan(), "{name}: x is NaN");
            assert!(!c.y().is_nan(), "{name}: y is NaN");
            assert!(!c.luminance().is_nan(), "{name}: luminance is NaN");
            assert!((0.0..=1.0).contains(&c.x()), "{name}: x out of [0..1]: {}", c.x());
            assert!((0.0..=1.0).contains(&c.y()), "{name}: y out of [0..1]: {}", c.y());
            assert!(c.luminance() >= 0.0, "{name}: negative luminance");
        }
    }

    #[test]
    fn rgb_black_becomes_degenerate_zero_point() {
        let c = CieColor::from_rgb(0.0, 0.0, 0.0);
        assert_approx_eq("x", c.x(), 0.0, 0.0);
        assert_approx_eq("y", c.y(), 0.0, 0.0);
        assert_approx_eq("Y", c.luminance(), 0.0, 0.0);

        let rgb = c.to_rgb();
        assert_rgb_in_0_1("black -> to_rgb range", rgb);
        assert_approx_eq("r", rgb.0, 0.0, 1e-6);
        assert_approx_eq("g", rgb.1, 0.0, 1e-6);
        assert_approx_eq("b", rgb.2, 0.0, 1e-6);
    }

    #[test]
    fn to_rgb_handles_y_equal_zero_without_divide_by_zero_and_clamps() {
        let c = CieColor::from_xy_lum(0.3, 0.0, 50.0);
        let rgb = c.to_rgb();
        assert_rgb_in_0_1("y=0 clamp", rgb);
        assert!(!rgb.0.is_nan() && !rgb.1.is_nan() && !rgb.2.is_nan());
    }

    #[test]
    fn luminance_scales_brightness_more_than_chromaticity() {
        let base = CieColor::from_rgb(1.0, 0.5, 0.0);
        let dimmer = CieColor::from_xy_lum(base.x(), base.y(), base.luminance() * 0.25);

        let (r1, g1, b1) = base.to_rgb();
        let (r2, g2, b2) = dimmer.to_rgb();

        assert!(r2 <= r1 + 1e-6, "red channel didn't dim as expected");
        assert!(g2 <= g1 + 1e-6, "green channel didn't dim as expected");
        assert!(b2 <= b1 + 1e-6, "blue channel didn't dim as expected");

        // Chromaticity should remain essentially the same.
        assert_approx_eq("x chromaticity stable", dimmer.x(), base.x(), 1e-6);
        assert_approx_eq("y chromaticity stable", dimmer.y(), base.y(), 1e-6);
    }

    #[test]
    fn to_rgb_clamps_out_of_gamut_or_invalid_xyy_inputs() {
        let bad = [
            ("x+y>1", CieColor::from_xy_lum(0.9, 0.9, 100.0)),
            ("negative luminance", CieColor::from_xy_lum(0.3, 0.3, -10.0)),
            ("negative x", CieColor::from_xy_lum(-0.1, 0.3, 50.0)),
            ("negative y", CieColor::from_xy_lum(0.3, -0.1, 50.0)),
        ];

        for (name, c) in bad {
            let rgb = c.to_rgb();
            assert!(!rgb.0.is_nan() && !rgb.1.is_nan() && !rgb.2.is_nan(), "{name}: NaN");
            assert_rgb_in_0_1(name, rgb);
        }
    }
}
