use std::{
    fmt, ops,
    str::{self, FromStr},
};

use facet_xml as xml;
use uuid::Uuid;

use crate::FileName;

#[derive(facet::Facet, Debug, Clone, PartialEq)]
#[facet(rename = "GeneralSceneDescription")]
pub struct GeneralSceneDescription {
    #[facet(xml::attribute, rename = "verMajor")]
    ver_major: i64,
    #[facet(xml::attribute, rename = "verMinor")]
    ver_minor: i64,
    #[facet(xml::attribute, rename = "provider", default = "")]
    provider: String,
    #[facet(xml::attribute, rename = "providerVersion", default = "")]
    provider_version: String,

    #[facet(rename = "UserData", default)]
    user_data: UserData,
    #[facet(rename = "Scene", default)]
    scene: Scene,
}

impl GeneralSceneDescription {
    pub fn ver_major(&self) -> i64 {
        self.ver_major
    }

    pub fn ver_minor(&self) -> i64 {
        self.ver_minor
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn provider_version(&self) -> &str {
        &self.provider_version
    }

    pub fn user_data(&self) -> &UserData {
        &self.user_data
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct UserData {
    /// The data is stored as raw XML markup because its structure may be ambiguous or application-specific.
    /// The user is responsible for parsing or interpreting the contents as needed.
    #[facet(rename = "Data")]
    data: Vec<xml::RawMarkup>,
}

impl UserData {
    pub fn data(&self) -> &[xml::RawMarkup] {
        &self.data
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Scene {
    #[facet(rename = "AUXData", default)]
    aux_data: AuxData,
    #[facet(rename = "Layers", default)]
    layers: Layers,
}

impl Scene {
    pub fn aux_data(&self) -> &AuxData {
        &self.aux_data
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct AuxData {
    #[facet(rename = "Symdef")]
    symdefs: Vec<Symdef>,
    #[facet(rename = "Position")]
    positions: Vec<Position>,
    #[facet(rename = "MappingDefinition")]
    mapping_definitions: Vec<MappingDefinition>,
    #[facet(rename = "Class")]
    class: Option<Class>,
}

impl AuxData {
    pub fn symdefs(&self) -> &[Symdef] {
        &self.symdefs
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    pub fn mapping_definitions(&self) -> &[MappingDefinition] {
        &self.mapping_definitions
    }

    pub fn class(&self) -> Option<&Class> {
        self.class.as_ref()
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Class {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    name: String,
}

impl Class {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Position {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    name: String,
}

impl Position {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Symdef {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    name: String,

    #[facet(rename = "ChildList", default)]
    child_list: SymdefChildList,
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct SymdefChildList {
    #[facet(rename = "Geometry3D")]
    geometry3ds: Vec<Geometry3D>,
    #[facet(rename = "Symbol")]
    symbols: Vec<Symbol>,
}

impl Symdef {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geometry3ds(&self) -> &[Geometry3D] {
        &self.child_list.geometry3ds
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.child_list.symbols
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct MappingDefinition {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    name: String,

    #[facet(rename = "SizeX")]
    size_x: SizeX,
    #[facet(rename = "SizeY")]
    size_y: SizeY,

    // FIXME: I can't seem to figure out how to directly parse this enum
    // using facet for some reason...
    #[facet(rename = "ScaleHandeling", default = "ScaleKeepRatio")]
    scale_handeling: String,

    #[facet(rename = "Source")]
    source: Source,
}

impl MappingDefinition {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size_x(&self) -> i64 {
        *self.size_x
    }

    pub fn size_y(&self) -> i64 {
        *self.size_y
    }

    pub fn scale_handeling(&self) -> ScaleHandeling {
        match self.scale_handeling.as_str() {
            "ScaleKeepRatio" => ScaleHandeling::ScaleKeepRatio,
            "ScaleIgnoreRatio" => ScaleHandeling::ScaleIgnoreRatio,
            "KeepSizeCenter" => ScaleHandeling::KeepSizeCenter,
            _ => panic!("invalid ScaleHandeling"),
        }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// `ScaleHandeling` is intentionally misspelled here to match the specification.
/// Although the correct spelling is `ScaleHandling`, we keep the spec's spelling for consistency.
#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
#[facet(default)]
pub enum ScaleHandeling {
    #[default]
    ScaleKeepRatio,
    ScaleIgnoreRatio,
    KeepSizeCenter,
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Eq)]
pub struct Source {
    #[facet(xml::attribute, rename = "linkedGeometry")]
    linked_geometry: String,
    #[facet(xml::attribute, rename = "type")]
    type_: SourceType,

    #[facet(xml::text)]
    value: String,
}

impl Source {
    pub fn linked_geometry(&self) -> &str {
        &self.linked_geometry
    }

    pub fn type_(&self) -> SourceType {
        self.type_
    }

    /// - If type is NDI or CITP, this is the Stream Name.
    /// - If type is File, this is the filename in MVR file.
    /// - If type is CaptureDevice, this is the CaptureDevice Name.
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum SourceType {
    Ndi,
    File,
    Citp,
    CaptureDevice,
}

#[cfg(not(tarpaulin_include))]
impl FromStr for SourceType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NDI" => Ok(SourceType::Ndi),
            "File" => Ok(SourceType::File),
            "CITP" => Ok(SourceType::Citp),
            "CaptureDevice" => Ok(SourceType::CaptureDevice),
            s => Err(crate::Error::InvalidSourceType(s.to_string())),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SourceType::Ndi => "NDI",
            SourceType::File => "File",
            SourceType::Citp => "CITP",
            SourceType::CaptureDevice => "CaptureDevice",
        };
        write!(f, "{}", s)
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Geometry3D {
    #[facet(xml::attribute, rename = "fileName")]
    file_name: FileName,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(rename = "Matrix")]
    matrix: Option<String>,
}

impl Geometry3D {
    pub fn file_name(&self) -> &FileName {
        &self.file_name
    }

    pub fn matrix(&self) -> Option<Matrix4x3> {
        self.matrix
            .as_ref()
            .and_then(|s| Matrix4x3::from_str(s).ok())
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Symbol {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "symdef", default = "")]
    symdef: Uuid,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(rename = "Matrix")]
    matrix: Option<String>,
}

impl Symbol {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn symdef(&self) -> Uuid {
        self.symdef
    }

    pub fn matrix(&self) -> Option<Matrix4x3> {
        self.matrix
            .as_ref()
            .and_then(|s| Matrix4x3::from_str(s).ok())
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Layers {
    #[facet(rename = "Layer")]
    layers: Vec<Layer>,
}

impl ops::Deref for Layers {
    type Target = [Layer];

    fn deref(&self) -> &Self::Target {
        &self.layers
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Layer {
    #[facet(xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    name: String,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(flatten, rename = "Matrix")]
    matrix: Option<String>,
}

impl Layer {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn matrix(&self) -> Option<Matrix4x3> {
        self.matrix
            .as_ref()
            .and_then(|s| Matrix4x3::from_str(s).ok())
    }
}

#[derive(Debug, Clone, PartialEq, facet::Facet)]
pub struct Matrix4x3 {
    u1: f64,
    u2: f64,
    u3: f64,
    v1: f64,
    v2: f64,
    v3: f64,
    w1: f64,
    w2: f64,
    w3: f64,
    o1: f64,
    o2: f64,
    o3: f64,
}

impl FromStr for Matrix4x3 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rest = s.trim();
        let mut groups = [[0.0; 3]; 4];

        for group in &mut groups {
            rest = rest.trim_start();
            if !rest.starts_with('{') {
                return Err(crate::Error::MatrixParseError(
                    "Missing opening brace".into(),
                ));
            }
            rest = &rest[1..];

            let end = rest
                .find('}')
                .ok_or_else(|| crate::Error::MatrixParseError("Missing closing brace".into()))?;

            let content = &rest[..end];
            rest = &rest[end + 1..];

            let parts: Vec<&str> = content.split(',').collect();
            if parts.len() != 3 {
                return Err(crate::Error::MatrixParseError(
                    "Expected 3 items per group".into(),
                ));
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

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SizeX(i64);

#[cfg(not(tarpaulin_include))]
impl From<i64> for SizeX {
    fn from(val: i64) -> Self {
        SizeX(val)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<SizeX> for i64 {
    fn from(val: SizeX) -> i64 {
        val.0
    }
}

#[cfg(not(tarpaulin_include))]
impl ops::Deref for SizeX {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for SizeX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(not(tarpaulin_include))]
impl str::FromStr for SizeX {
    type Err = <i64 as str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(SizeX)
    }
}

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SizeY(i64);

#[cfg(not(tarpaulin_include))]
impl From<i64> for SizeY {
    fn from(val: i64) -> Self {
        SizeY(val)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<SizeY> for i64 {
    fn from(val: SizeY) -> i64 {
        val.0
    }
}

#[cfg(not(tarpaulin_include))]
impl ops::Deref for SizeY {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for SizeY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(not(tarpaulin_include))]
impl str::FromStr for SizeY {
    type Err = <i64 as str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>().map(SizeY)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::MvrFile;

    fn expected_gsd() -> GeneralSceneDescription {
        GeneralSceneDescription {
            ver_major: 1,
            ver_minor: 5,
            provider: "Provider".to_string(),
            provider_version: "Provider Version".to_string(),
            user_data: UserData {
                data: vec![
                    xml::RawMarkup::from(r#"<Data provider="Data Provider 1" ver="0.1" />"#),
                    xml::RawMarkup::from(
                        r#"<Data provider="Data Provider 2"><VWEntry key="ce7c4eda-1c47-4b41-af56-530116c475b2">Custom Entry</VWEntry></Data>"#,
                    ),
                ],
            },
            scene: Scene {
                aux_data: AuxData {
                    class: Some(Class {
                        uuid: Uuid::parse_str("4157c914-094b-4808-87ee-dd7ebd6f9f97").unwrap(),
                        name: "Class Name".to_string(),
                    }),
                    positions: vec![
                        Position {
                            uuid: Uuid::parse_str("48444956-9b0d-11f0-a3e9-dc567b68abae").unwrap(),
                            name: "Position Name 1".to_string(),
                        },
                        Position {
                            uuid: Uuid::parse_str("56b76b02-14ee-4309-bd58-0961493e93e3").unwrap(),
                            name: "".to_string(),
                        },
                    ],
                    symdefs: vec![
                        Symdef {
                            uuid: Uuid::parse_str("317a5549-659d-42a8-9cdb-5e1a411560c1").unwrap(),
                            name: "Symdef Name 1".to_string(),
                            child_list: SymdefChildList {
                                geometry3ds: vec![Geometry3D {
                                    file_name: FileName::new("geometry_file.glb").unwrap(),
                                    matrix: Some("{1,2,3}{4,5,6}{7,8,9}{10,11,12}".to_string()),
                                }],
                                symbols: vec![],
                            },
                        },
                        Symdef {
                            uuid: Uuid::parse_str("0584afe1-2cbc-4a98-b5d2-2261aafdbdbb").unwrap(),
                            name: "Symdef Name 2".to_string(),
                            child_list: SymdefChildList {
                                geometry3ds: vec![Geometry3D {
                                    file_name: FileName::new("geometry_file.glb").unwrap(),
                                    matrix: None,
                                }],
                                symbols: vec![],
                            },
                        },
                        Symdef {
                            uuid: Uuid::parse_str("0f76c345-0f3f-4251-8e19-8dc0690ffd6f").unwrap(),
                            name: "Symdef Name 3".to_string(),
                            child_list: SymdefChildList {
                                geometry3ds: vec![],
                                symbols: vec![Symbol {
                                    uuid: Uuid::parse_str("4de1d6e2-5437-4ec3-949e-2065cb4fbfce")
                                        .unwrap(),
                                    symdef: Uuid::parse_str("4dd4be9e-ba5c-4ffb-90be-0419b4d977a4")
                                        .unwrap(),
                                    matrix: None,
                                }],
                            },
                        },
                        Symdef {
                            uuid: Uuid::parse_str("a1907a3e-16c1-4702-984a-9de0b41adff4").unwrap(),
                            name: "".to_string(),
                            child_list: SymdefChildList {
                                geometry3ds: vec![],
                                symbols: vec![Symbol {
                                    uuid: Uuid::parse_str("f7199cb8-e6f9-493d-8d52-7cf529453fc4")
                                        .unwrap(),
                                    symdef: Uuid::parse_str("aa517032-d1f1-40d4-b14d-63ed6527349f")
                                        .unwrap(),
                                    matrix: Some("{1,2,3}{4,5,6}{7,8,9}{10,11,12}".to_string()),
                                }],
                            },
                        },
                    ],
                    mapping_definitions: vec![MappingDefinition {
                        uuid: Uuid::parse_str("bef95eb8-98ac-4217-b10d-fb4b83381398").unwrap(),
                        name: "Mapping Definition Name 1".to_string(),
                        size_x: SizeX(1920),
                        size_y: SizeY(1080),
                        scale_handeling: "ScaleIgnoreRatio".to_string(),
                        source: Source {
                            linked_geometry: "linked_geometry".to_string(),
                            type_: SourceType::CaptureDevice,
                            value: "value".to_string(),
                        },
                    }],
                },
                layers: Layers { layers: vec![] },
            },
        }
    }

    fn load_gsd() -> GeneralSceneDescription {
        MvrFile::load_from_file("tests/mvr/sample_show.mvr")
            .expect("Should load MvrFile")
            .general_scene_description
    }

    #[test]
    fn test_load_mvr_header() {
        let loaded = load_gsd();
        let expected = expected_gsd();

        assert_eq!(loaded.ver_major(), expected.ver_major());
        assert_eq!(loaded.ver_minor(), expected.ver_minor());
        assert_eq!(loaded.provider(), expected.provider());
        assert_eq!(loaded.provider_version(), expected.provider_version());
    }

    #[test]
    fn test_load_mvr_user_data() {
        let loaded = load_gsd();
        let expected = expected_gsd();

        assert_eq!(loaded.user_data().data(), expected.user_data().data());
    }

    #[test]
    fn test_load_mvr_aux_data() {
        let loaded = load_gsd();
        let expected = expected_gsd();

        let loaded_aux = loaded.scene().aux_data();
        let expected_aux = expected.scene().aux_data();

        // Classes.
        let loaded_class = loaded_aux.class();
        let gsd_class = expected_aux.class();
        match (loaded_class, gsd_class) {
            (Some(a), Some(b)) => {
                assert_eq!(a.uuid(), b.uuid());
                assert_eq!(a.name(), b.name());
            }
            (None, None) => {}
            _ => panic!("Class mismatch"),
        }

        // Positions.
        let loaded_positions = loaded_aux.positions();
        let gsd_positions = expected_aux.positions();
        assert_eq!(loaded_positions.len(), gsd_positions.len());
        for (a, b) in loaded_positions.iter().zip(gsd_positions.iter()) {
            assert_eq!(a.uuid(), b.uuid());
            assert_eq!(a.name(), b.name());
        }

        // Symdefs.
        let loaded_symdefs = loaded_aux.symdefs();
        let gsd_symdefs = expected_aux.symdefs();
        assert_eq!(loaded_symdefs.len(), gsd_symdefs.len());
        for (a, b) in loaded_symdefs.iter().zip(gsd_symdefs.iter()) {
            assert_eq!(a.uuid(), b.uuid());
            assert_eq!(a.name(), b.name());

            let a_geometry3ds = a.geometry3ds();
            let b_geometry3ds = b.geometry3ds();
            assert_eq!(a_geometry3ds.len(), b_geometry3ds.len());
            for (ag, bg) in a_geometry3ds.iter().zip(b_geometry3ds.iter()) {
                assert_eq!(ag.file_name(), bg.file_name());
                assert_eq!(ag.matrix(), bg.matrix());
            }

            let a_symbols = a.symbols();
            let b_symbols = b.symbols();
            assert_eq!(a_symbols.len(), b_symbols.len());
            for (asym, bsym) in a_symbols.iter().zip(b_symbols.iter()) {
                assert_eq!(asym.uuid(), bsym.uuid());
                assert_eq!(asym.symdef(), bsym.symdef());
                assert_eq!(asym.matrix(), bsym.matrix());
            }
        }

        // Mapping Definitions.
        let loaded_maps = loaded_aux.mapping_definitions();
        let gsd_maps = expected_aux.mapping_definitions();
        assert_eq!(loaded_maps.len(), gsd_maps.len());
        for (a, b) in loaded_maps.iter().zip(gsd_maps.iter()) {
            assert_eq!(a.uuid(), b.uuid());
            assert_eq!(a.name(), b.name());
            assert_eq!(a.size_x(), b.size_x());
            assert_eq!(a.size_y(), b.size_y());
            assert_eq!(a.scale_handeling(), b.scale_handeling());

            assert_eq!(a.source().linked_geometry(), b.source().linked_geometry());
            assert_eq!(a.source().type_(), b.source().type_());
            assert_eq!(a.source().value(), b.source().value());
        }
    }

    #[test]
    #[ignore]
    fn test_load_mvr_layers() {
        let gsd = expected_gsd();
        let loaded = load_gsd();

        let loaded_scene = loaded.scene();
        let gsd_scene = gsd.scene();

        let loaded_layers = loaded_scene.layers();
        let gsd_layers = gsd_scene.layers();
        assert_eq!(loaded_layers.len(), gsd_layers.len());
        for (a, b) in loaded_layers.iter().zip(gsd_layers.iter()) {
            assert_eq!(a.uuid(), b.uuid());
            assert_eq!(a.name(), b.name());
            assert_eq!(a.matrix(), b.matrix());
        }
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
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "    ";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{}{}{}{}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{}{7,8,9}{10,11,12}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{4,5,6}{7,8,9}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12,13}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,foo}{4,5,6}{7,8,9}{10,11,12}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{4,5,6}{7,8,9{10,11,12}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "1,2,3,4,5,6,7,8,9,10,11,12";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}4,5,6}{7,8,9}{10,11,12}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12}}";
        assert!(matches!(
            Matrix4x3::from_str(s),
            Err(crate::Error::MatrixParseError(_))
        ));
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
