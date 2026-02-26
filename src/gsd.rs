use std::{fmt, ops, str::FromStr};

use facet_xml as xml;
use uuid::Uuid;

#[derive(facet::Facet, Debug, Clone, PartialEq)]
#[facet(rename = "GeneralSceneDescription")]
pub struct GeneralSceneDescription {
    #[facet(xml::attribute, rename = "verMajor")]
    ver_major: i32,
    #[facet(xml::attribute, rename = "verMinor")]
    ver_minor: i32,
    #[facet(xml::attribute, rename = "provider", default = "")]
    provider: String,
    #[facet(xml::attribute, rename = "providerVersion", default = "")]
    provider_version: String,

    #[facet(rename = "UserData")]
    user_data: Option<UserData>,
    #[facet(rename = "Scene")]
    scene: Scene,
}

impl GeneralSceneDescription {
    pub fn ver_major(&self) -> i32 {
        self.ver_major
    }

    pub fn ver_minor(&self) -> i32 {
        self.ver_minor
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn provider_version(&self) -> &str {
        &self.provider_version
    }

    pub fn user_data(&self) -> Option<&UserData> {
        self.user_data.as_ref()
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
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
    #[facet(rename = "AUXData")]
    aux_data: Option<AuxData>,
    #[facet(rename = "Layers", default)]
    layers: Layers,
}

impl Scene {
    pub fn aux_data(&self) -> Option<&AuxData> {
        self.aux_data.as_ref()
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct AuxData {
    #[facet(rename = "Symdef")]
    symdefs: Vec<Symdef>,
    #[facet(rename = "Position")]
    positions: Vec<Position>,
    #[facet(rename = "MappingDefinition")]
    mapping_definitions: Vec<MappingDefinition>,
    #[facet(rename = "Class")]
    classes: Vec<Class>,
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

    pub fn classes(&self) -> &[Class] {
        &self.classes
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

    #[facet(rename = "ChildList")]
    child_list: SymdefChildList,
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
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
    size_x: i32,
    #[facet(rename = "SizeY")]
    size_y: i32,

    // FIXME: I can't seem to figure out how to directly parse this enum
    // using facet for some reason...
    #[facet(rename = "ScaleHandeling", default = "ScaleKeepRatio")]
    scale_handeling: String,

    #[facet(xml::text)]
    source: Option<String>,
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

impl MappingDefinition {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size_x(&self) -> i32 {
        self.size_x
    }

    pub fn size_y(&self) -> i32 {
        self.size_y
    }

    pub fn scale_handeling(&self) -> ScaleHandeling {
        match self.scale_handeling.as_str() {
            "ScaleKeepRatio" => ScaleHandeling::ScaleKeepRatio,
            "ScaleIgnoreRatio" => ScaleHandeling::ScaleIgnoreRatio,
            "KeepSizeCenter" => ScaleHandeling::KeepSizeCenter,
            _ => panic!("invalid ScaleHandeling"),
        }
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
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

    pub fn matrix(&self) -> Option<Matrix> {
        self.matrix.as_ref().and_then(|s| Matrix::from_str(s).ok())
    }
}

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Geometry3D {
    #[facet(xml::attribute, rename = "fileName")]
    file_name: String,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(rename = "Matrix")]
    matrix: Option<String>,
}

impl Geometry3D {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn matrix(&self) -> Option<Matrix> {
        self.matrix.as_ref().and_then(|s| Matrix::from_str(s).ok())
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

    pub fn matrix(&self) -> Option<Matrix> {
        self.matrix.as_ref().and_then(|s| Matrix::from_str(s).ok())
    }
}

#[derive(Debug, Clone, PartialEq, facet::Facet)]
pub struct Matrix {
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

impl FromStr for Matrix {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(char::is_whitespace, "");

        let mut values = Vec::new();
        let mut rest = s.as_str();
        while let Some(start) = rest.find('{') {
            let rest2 = &rest[start + 1..];
            if let Some(end) = rest2.find('}') {
                let group = &rest2[..end];
                for num in group.split(',') {
                    if num.is_empty() {
                        return Err(crate::Error::MatrixParseError(
                            "Empty value in matrix group".to_string(),
                        ));
                    }
                    let val: f64 = num.parse().map_err(|_| {
                        crate::Error::MatrixParseError(format!("Failed to parse '{}' as f64", num))
                    })?;
                    values.push(val);
                }
                rest = &rest2[end + 1..];
            } else {
                return Err(crate::Error::MatrixParseError(
                    "Mismatched '{' in matrix string".to_string(),
                ));
            }
        }

        if values.len() != 12 {
            return Err(crate::Error::MatrixParseError(format!(
                "Expected 12 values for Matrix, got {}",
                values.len()
            )));
        }

        Ok(Matrix {
            u1: values[0],
            u2: values[1],
            u3: values[2],
            v1: values[3],
            v2: values[4],
            v3: values[5],
            w1: values[6],
            w2: values[7],
            w3: values[8],
            o1: values[9],
            o2: values[10],
            o3: values[11],
        })
    }
}

impl fmt::Display for Matrix {
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
            user_data: Some(UserData {
                data: vec![
                    xml::RawMarkup::from(r#"<Data provider="Data Provider 1" ver="0.1" />"#),
                    xml::RawMarkup::from(
                        r#"<Data provider="Data Provider 2"><VWEntry key="ce7c4eda-1c47-4b41-af56-530116c475b2">Custom Entry</VWEntry></Data>"#,
                    ),
                ],
            }),
            scene: Scene {
                aux_data: Some(AuxData {
                    classes: vec![Class {
                        uuid: Uuid::parse_str("4157c914-094b-4808-87ee-dd7ebd6f9f97").unwrap(),
                        name: "Class Name".to_string(),
                    }],
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
                                    file_name: "geometry_file.glb".to_string(),
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
                                    file_name: "geometry_file.glb".to_string(),
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
                        size_x: 1920,
                        size_y: 1080,
                        scale_handeling: "ScaleIgnoreRatio".to_string(),
                        source: Some("movie.mov".to_string()),
                    }],
                }),
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
        let gsd = expected_gsd();
        let loaded = load_gsd();

        assert_eq!(loaded.ver_major(), gsd.ver_major());
        assert_eq!(loaded.ver_minor(), gsd.ver_minor());
        assert_eq!(loaded.provider(), gsd.provider());
        assert_eq!(loaded.provider_version(), gsd.provider_version());
    }

    #[test]
    fn test_load_mvr_user_data() {
        let gsd = expected_gsd();
        let loaded = load_gsd();

        let loaded_user_data = loaded.user_data();
        let gsd_user_data = gsd.user_data();
        assert_eq!(loaded_user_data.is_some(), gsd_user_data.is_some());
        if let (Some(loaded_ud), Some(gsd_ud)) = (loaded_user_data, gsd_user_data) {
            assert_eq!(loaded_ud.data(), gsd_ud.data());
        }
    }

    #[test]
    fn test_load_mvr_aux_data() {
        let gsd = expected_gsd();
        let loaded = load_gsd();

        let loaded_scene = loaded.scene();
        let gsd_scene = gsd.scene();

        let loaded_aux = loaded_scene.aux_data();
        let gsd_aux = gsd_scene.aux_data();
        assert_eq!(loaded_aux.is_some(), gsd_aux.is_some());
        if let (Some(loaded_aux), Some(gsd_aux)) = (loaded_aux, gsd_aux) {
            // Classes.
            let loaded_classes = loaded_aux.classes();
            let gsd_classes = gsd_aux.classes();
            assert_eq!(loaded_classes.len(), gsd_classes.len());
            for (a, b) in loaded_classes.iter().zip(gsd_classes.iter()) {
                assert_eq!(a.uuid(), b.uuid());
                assert_eq!(a.name(), b.name());
            }
            // Positions.
            let loaded_positions = loaded_aux.positions();
            let gsd_positions = gsd_aux.positions();
            assert_eq!(loaded_positions.len(), gsd_positions.len());
            for (a, b) in loaded_positions.iter().zip(gsd_positions.iter()) {
                assert_eq!(a.uuid(), b.uuid());
                assert_eq!(a.name(), b.name());
            }
            // Symdefs.
            let loaded_symdefs = loaded_aux.symdefs();
            let gsd_symdefs = gsd_aux.symdefs();
            assert_eq!(loaded_symdefs.len(), gsd_symdefs.len());
            for (a, b) in loaded_symdefs.iter().zip(gsd_symdefs.iter()) {
                assert_eq!(a.uuid(), b.uuid());
                assert_eq!(a.name(), b.name());

                // Test Geometry3D child list
                let a_geometry3ds = a.geometry3ds();
                let b_geometry3ds = b.geometry3ds();
                assert_eq!(a_geometry3ds.len(), b_geometry3ds.len());
                for (ag, bg) in a_geometry3ds.iter().zip(b_geometry3ds.iter()) {
                    assert_eq!(ag.file_name(), bg.file_name());
                    assert_eq!(ag.matrix(), bg.matrix());
                }

                // Test Symbol child list
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
            let gsd_maps = gsd_aux.mapping_definitions();
            assert_eq!(loaded_maps.len(), gsd_maps.len());
            for (a, b) in loaded_maps.iter().zip(gsd_maps.iter()) {
                assert_eq!(a.uuid(), b.uuid());
                assert_eq!(a.name(), b.name());
                assert_eq!(a.size_x(), b.size_x());
                assert_eq!(a.size_y(), b.size_y());
                assert_eq!(a.scale_handeling(), b.scale_handeling());
                assert_eq!(a.source(), b.source());
            }
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
        let m = Matrix::from_str(s).unwrap();
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
        let m = Matrix::from_str(s).unwrap();
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

        let s = "{1,2,3}{4,5,6}{7,8,9}";
        assert!(Matrix::from_str(s).is_err());

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12,13}";
        assert!(Matrix::from_str(s).is_err());

        let s = "{1,2,foo}{4,5,6}{7,8,9}{10,11,12}";
        assert!(Matrix::from_str(s).is_err());

        let s = "{1,2,3}{4,5,6}{7,8,9{10,11,12}";
        assert!(Matrix::from_str(s).is_err());
    }

    #[test]
    #[rustfmt::skip]
    fn test_display_matrix() {
        let m = Matrix {
            u1: 1.0,  u2: 2.0,  u3: 3.0,
            v1: 4.0,  v2: 5.0,  v3: 6.0,
            w1: 7.0,  w2: 8.0,  w3: 9.0,
            o1: 10.0, o2: 11.0, o3: 12.0,
        };
        let s = m.to_string();
        assert_eq!(s, "{1,2,3}{4,5,6}{7,8,9}{10,11,12}".to_string());

        let m2 = Matrix::from_str(&s).unwrap();
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
