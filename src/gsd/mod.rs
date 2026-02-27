use std::{
    fmt,
    str::{self, FromStr},
};

use crate::FileName;

mod aux_data;
mod layers;
mod scene;
mod user_data;

pub use aux_data::*;
pub use layers::*;
pub use scene::*;
pub use user_data::*;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "GeneralSceneDescription")]
pub struct GeneralSceneDescription {
    #[serde(rename = "@verMajor")]
    pub(crate) ver_major: i64,
    #[serde(rename = "@verMinor")]
    pub(crate) ver_minor: i64,
    #[serde(rename = "@provider", default)]
    pub(crate) provider: Option<String>,
    #[serde(rename = "@providerVersion", default)]
    pub(crate) provider_version: Option<String>,

    #[serde(rename = "UserData", default)]
    pub(crate) user_data: UserData,
    #[serde(rename = "Scene", default)]
    pub(crate) scene: Scene,
}

impl GeneralSceneDescription {
    pub fn ver_major(&self) -> i64 {
        self.ver_major
    }

    pub fn ver_minor(&self) -> i64 {
        self.ver_minor
    }

    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    pub fn provider_version(&self) -> Option<&str> {
        self.provider_version.as_deref()
    }

    pub fn user_data(&self) -> &UserData {
        &self.user_data
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Matrix", try_from = "String", into = "String")]
pub struct Matrix4x3 {
    pub(crate) u1: f64,
    pub(crate) u2: f64,
    pub(crate) u3: f64,
    pub(crate) v1: f64,
    pub(crate) v2: f64,
    pub(crate) v3: f64,
    pub(crate) w1: f64,
    pub(crate) w2: f64,
    pub(crate) w3: f64,
    pub(crate) o1: f64,
    pub(crate) o2: f64,
    pub(crate) o3: f64,
}

impl FromStr for Matrix4x3 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rest = s.trim();
        let mut groups = [[0.0; 3]; 4];

        for group in &mut groups {
            rest = rest.trim_start();
            if !rest.starts_with('{') {
                return Err(crate::Error::MatrixParseError("Missing opening brace".into()));
            }
            rest = &rest[1..];

            let end = rest
                .find('}')
                .ok_or_else(|| crate::Error::MatrixParseError("Missing closing brace".into()))?;

            let content = &rest[..end];
            rest = &rest[end + 1..];

            let parts: Vec<&str> = content.split(',').collect();
            if parts.len() != 3 {
                return Err(crate::Error::MatrixParseError("Expected 3 items per group".into()));
            }

            for (i, part) in parts.iter().enumerate() {
                group[i] = part
                    .trim()
                    .parse::<f64>()
                    .map_err(|_| crate::Error::MatrixParseError("Invalid float".into()))?;
            }
        }

        if !rest.trim().is_empty() {
            return Err(crate::Error::MatrixParseError("Trailing characters".into()));
        }

        Ok(Matrix4x3 {
            u1: groups[0][0],
            u2: groups[0][1],
            u3: groups[0][2],
            v1: groups[1][0],
            v2: groups[1][1],
            v3: groups[1][2],
            w1: groups[2][0],
            w2: groups[2][1],
            w3: groups[2][2],
            o1: groups[3][0],
            o2: groups[3][1],
            o3: groups[3][2],
        })
    }
}

impl fmt::Display for Matrix4x3 {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}",
            self.u1, self.u2, self.u3,
            self.v1, self.v2, self.v3,
            self.w1, self.w2, self.w3,
            self.o1, self.o2, self.o3
        )
    }
}

pub(crate) fn deserialize_matrix_option<'de, D>(
    deserializer: D,
) -> Result<Option<Matrix4x3>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize as _;
    match Option::<Matrix4x3>::deserialize(deserializer) {
        Ok(val) => Ok(val),
        Err(_) => Ok(None),
    }
}

#[cfg(not(tarpaulin_include))]
impl From<Matrix4x3> for String {
    fn from(matrix: Matrix4x3) -> Self {
        matrix.to_string()
    }
}

#[cfg(not(tarpaulin_include))]
impl TryFrom<String> for Matrix4x3 {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Matrix4x3::from_str(&value)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Gobo")]
pub struct Gobo {
    #[serde(rename = "@rotation", default = "default_rotation")]
    pub(crate) rotation: f64,

    #[serde(rename = "$value")]
    pub(crate) file_name: FileName,
}

fn default_rotation() -> f64 {
    0.0
}

impl Gobo {
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn file_name(&self) -> &FileName {
        &self.file_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_gsd() {
        let xml = r#"
            <GeneralSceneDescription provider="Provider" providerVersion="Provider Version" verMajor="1" verMinor="5">
                <UserData>
                    <Data provider="Data Provider 1" ver="0.1" />
                </UserData>
                <Scene>
                    <AUXData>
                        <Class name="Class 1" uuid="4157c914-094b-4808-87ee-dd7ebd6f9f97" />
                    </AUXData>
                    <Layers>
                        <Layer name="Layer 1" uuid="7bfe64c8-8a96-402b-a1dc-85395f93890b" />
                    </Layers>
                </Scene>
            </GeneralSceneDescription>
        "#;

        let gsd: GeneralSceneDescription = quick_xml::de::from_str(xml).unwrap();
        let user_data = gsd.user_data();
        let user_data_entries = user_data.data();
        let scene = gsd.scene();
        let aux_data = scene.aux_data();
        let class = &aux_data.classes()[0];
        let layers = scene.layers();

        assert_eq!(gsd.ver_major(), 1);
        assert_eq!(gsd.ver_minor(), 5);
        assert_eq!(gsd.provider(), Some("Provider"));
        assert_eq!(gsd.provider_version(), Some("Provider Version"));
        assert_eq!(user_data_entries.len(), 1);
        assert_eq!(user_data_entries[0].provider(), "Data Provider 1");
        assert_eq!(user_data_entries[0].ver(), Some("0.1"));
        assert_eq!(class.name(), "Class 1");
        assert_eq!(class.uuid().to_string(), "4157c914-094b-4808-87ee-dd7ebd6f9f97");
        assert_eq!(layers[0].name(), "Layer 1");
    }

    #[test]
    fn test_parse_matrix() {
        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12}";
        let m = Matrix4x3::from_str(s).unwrap();
        assert_eq!(m.u1, 1.0);
        assert_eq!(m.u2, 2.0);
        assert_eq!(m.u3, 3.0);
        assert_eq!(m.v1, 4.0);
        assert_eq!(m.v2, 5.0);
        assert_eq!(m.v3, 6.0);
        assert_eq!(m.w1, 7.0);
        assert_eq!(m.w2, 8.0);
        assert_eq!(m.w3, 9.0);
        assert_eq!(m.o1, 10.0);
        assert_eq!(m.o2, 11.0);
        assert_eq!(m.o3, 12.0);

        let s = " { 1 , 2 , 3 } { 4 , 5 , 6 } { 7 , 8 , 9 } { 10 , 11 , 12 } ";
        let m = Matrix4x3::from_str(s).unwrap();
        assert_eq!(m.u1, 1.0);
        assert_eq!(m.u2, 2.0);
        assert_eq!(m.u3, 3.0);
        assert_eq!(m.v1, 4.0);
        assert_eq!(m.v2, 5.0);
        assert_eq!(m.v3, 6.0);
        assert_eq!(m.w1, 7.0);
        assert_eq!(m.w2, 8.0);
        assert_eq!(m.w3, 9.0);
        assert_eq!(m.o1, 10.0);
        assert_eq!(m.o2, 11.0);
        assert_eq!(m.o3, 12.0);

        let s = "";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "    ";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{}{}{}{}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{}{7,8,9}{10,11,12}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{4,5,6}{7,8,9}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12,13}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,foo}{4,5,6}{7,8,9}{10,11,12}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{4,5,6}{7,8,9{10,11,12}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "1,2,3,4,5,6,7,8,9,10,11,12";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}4,5,6}{7,8,9}{10,11,12}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12}}";
        assert!(matches!(Matrix4x3::from_str(s), Err(crate::Error::MatrixParseError(_))));
    }

    #[test]
    #[rustfmt::skip]
    fn test_display_matrix() {
        let m = Matrix4x3 {
            u1: 1.0,  u2: 2.0,  u3: 3.0,
            v1: 4.0,  v2: 5.0,  v3: 6.0,
            w1: 7.0,  w2: 8.0,  w3: 9.0,
            o1: 10.0, o2: 11.0, o3: 12.0,
        };
        let s = m.to_string();
        assert_eq!(s, "{1,2,3}{4,5,6}{7,8,9}{10,11,12}".to_string());

        let m2 = Matrix4x3::from_str(&s).unwrap();
        assert_eq!(m2.u1, 1.0);
        assert_eq!(m2.u2, 2.0);
        assert_eq!(m2.u3, 3.0);
        assert_eq!(m2.v1, 4.0);
        assert_eq!(m2.v2, 5.0);
        assert_eq!(m2.v3, 6.0);
        assert_eq!(m2.w1, 7.0);
        assert_eq!(m2.w2, 8.0);
        assert_eq!(m2.w3, 9.0);
        assert_eq!(m2.o1, 10.0);
        assert_eq!(m2.o2, 11.0);
        assert_eq!(m2.o3, 12.0);
    }
}
