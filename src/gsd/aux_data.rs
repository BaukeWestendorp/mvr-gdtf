use std::{fmt, ops, str};

use uuid::Uuid;

use crate::{FileName, Matrix4x3, deserialize_matrix_option};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "AuxData")]
pub struct AuxData {
    #[serde(rename = "Symdef", default)]
    pub(crate) symdefs: Vec<Symdef>,
    #[serde(rename = "Position", default)]
    pub(crate) positions: Vec<Position>,
    #[serde(rename = "MappingDefinition", default)]
    pub(crate) mapping_definitions: Vec<MappingDefinition>,
    #[serde(rename = "Class", default)]
    pub(crate) classes: Vec<Class>,
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
        self.classes.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Class")]
pub struct Class {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,
}

impl Class {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Position")]
pub struct Position {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,
}

impl Position {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Symdef")]
pub struct Symdef {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,

    #[serde(rename = "ChildList", default)]
    pub(crate) child_list: SymdefChildList,
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

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "SymdefChildList")]
pub struct SymdefChildList {
    #[serde(rename = "Geometry3D", default)]
    pub(crate) geometry3ds: Vec<Geometry3D>,
    #[serde(rename = "Symbol", default)]
    pub(crate) symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "MappingDefinition")]
pub struct MappingDefinition {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,

    #[serde(rename = "SizeX")]
    pub(crate) size_x: SizeX,
    #[serde(rename = "SizeY")]
    pub(crate) size_y: SizeY,
    #[serde(rename = "ScaleHandeling", default)]
    pub(crate) scale_handeling: ScaleHandeling,
    #[serde(rename = "Source")]
    pub(crate) source: Source,
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
        self.scale_handeling
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

/// `ScaleHandeling` is intentionally misspelled here to match the specification.
/// Although the correct spelling is `ScaleHandling`, we keep the spec's spelling for consistency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "ScaleHandeling")]
pub enum ScaleHandeling {
    #[default]
    ScaleKeepRatio,
    ScaleIgnoreRatio,
    KeepSizeCenter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Source")]
pub struct Source {
    #[serde(rename = "@linkedGeometry")]
    pub(crate) linked_geometry: String,
    #[serde(rename = "@type")]
    pub(crate) type_: SourceType,

    #[serde(rename = "$value")]
    pub(crate) value: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "SourceType")]
pub enum SourceType {
    #[serde(rename = "NDI")]
    Ndi,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "CITP")]
    Citp,
    #[serde(rename = "CaptureDevice")]
    CaptureDevice,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Geometry3D")]
pub struct Geometry3D {
    #[serde(rename = "@fileName")]
    pub(crate) file_name: FileName,

    #[serde(rename = "Matrix", default, deserialize_with = "deserialize_matrix_option")]
    pub(crate) matrix: Option<Matrix4x3>,
}

impl Geometry3D {
    pub fn file_name(&self) -> &FileName {
        &self.file_name
    }

    pub fn matrix(&self) -> Option<Matrix4x3> {
        self.matrix
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Symbol")]
pub struct Symbol {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@symdef", default)]
    pub(crate) symdef: Uuid,

    #[serde(rename = "Matrix", default, deserialize_with = "deserialize_matrix_option")]
    pub(crate) matrix: Option<Matrix4x3>,
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
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "SizeX")]
pub struct SizeX(pub(crate) i64);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "SizeY")]
pub struct SizeY(pub(crate) i64);

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
    use super::*;
    use uuid::Uuid;

    fn uuid(s: &str) -> Uuid {
        Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn test_aux_data_empty() {
        let xml = r#"<AuxData/>"#;
        let aux: AuxData = quick_xml::de::from_str(xml).unwrap();
        assert!(aux.symdefs().is_empty());
        assert!(aux.positions().is_empty());
        assert!(aux.mapping_definitions().is_empty());
        assert!(aux.classes().is_empty());
    }

    #[test]
    fn test_aux_data_with_all_children() {
        let xml = r#"
            <AuxData>
                <Symdef uuid="11111111-2222-3333-4444-555555555555" name="Symdef1">
                    <ChildList>
                        <Geometry3D fileName="geometry.obj"/>
                        <Symbol uuid="aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee" symdef="11111111-2222-3333-4444-555555555555"/>
                    </ChildList>
                </Symdef>
                <Position uuid="66666666-7777-8888-9999-aaaaaaaaaaaa" name="Position1"/>
                <MappingDefinition uuid="bbbbbbbb-cccc-dddd-eeee-ffffffffffff" name="Mapping1">
                    <SizeX>1920</SizeX>
                    <SizeY>1080</SizeY>
                    <Source linkedGeometry="geom" type="File">movie.mov</Source>
                    <ScaleHandeling>ScaleKeepRatio</ScaleHandeling>
                </MappingDefinition>
                <Class uuid="cccccccc-dddd-eeee-ffff-111111111111" name="Class1"/>
            </AuxData>
        "#;
        let aux: AuxData = quick_xml::de::from_str(xml).unwrap();

        // Symdef
        assert_eq!(aux.symdefs().len(), 1);
        let symdef = &aux.symdefs()[0];
        assert_eq!(symdef.uuid(), uuid("11111111-2222-3333-4444-555555555555"));
        assert_eq!(symdef.name(), "Symdef1");
        assert_eq!(symdef.geometry3ds().len(), 1);
        assert_eq!(symdef.geometry3ds()[0].file_name().as_str(), "geometry.obj");
        assert_eq!(symdef.symbols().len(), 1);
        assert_eq!(symdef.symbols()[0].uuid(), uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"));
        assert_eq!(symdef.symbols()[0].symdef(), uuid("11111111-2222-3333-4444-555555555555"));

        // Position
        assert_eq!(aux.positions().len(), 1);
        let pos = &aux.positions()[0];
        assert_eq!(pos.uuid(), uuid("66666666-7777-8888-9999-aaaaaaaaaaaa"));
        assert_eq!(pos.name(), "Position1");

        // MappingDefinition
        assert_eq!(aux.mapping_definitions().len(), 1);
        let mapping = &aux.mapping_definitions()[0];
        assert_eq!(mapping.uuid(), uuid("bbbbbbbb-cccc-dddd-eeee-ffffffffffff"));
        assert_eq!(mapping.name(), "Mapping1");
        assert_eq!(mapping.size_x(), 1920);
        assert_eq!(mapping.size_y(), 1080);
        assert_eq!(mapping.scale_handeling(), ScaleHandeling::ScaleKeepRatio);
        let src = mapping.source();
        assert_eq!(src.linked_geometry(), "geom");
        assert_eq!(src.type_(), SourceType::File);
        assert_eq!(src.value(), "movie.mov");

        // Class
        let class = &aux.classes()[0];
        assert_eq!(class.uuid(), uuid("cccccccc-dddd-eeee-ffff-111111111111"));
        assert_eq!(class.name(), "Class1");
    }

    #[test]
    fn test_symdef_childlist_multiple() {
        let xml = r#"
            <Symdef uuid="12345678-1234-1234-1234-1234567890ab" name="TestSymdef">
                <ChildList>
                    <Geometry3D fileName="geom1.obj"/>
                    <Geometry3D fileName="geom2.obj"/>
                    <Symbol uuid="87654321-4321-4321-4321-ba0987654321" symdef="12345678-1234-1234-1234-1234567890ab"/>
                </ChildList>
            </Symdef>
        "#;
        let symdef: Symdef = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(symdef.uuid(), uuid("12345678-1234-1234-1234-1234567890ab"));
        assert_eq!(symdef.name(), "TestSymdef");
        assert_eq!(symdef.geometry3ds().len(), 2);
        assert_eq!(symdef.geometry3ds()[0].file_name().as_str(), "geom1.obj");
        assert_eq!(symdef.geometry3ds()[1].file_name().as_str(), "geom2.obj");
        assert_eq!(symdef.symbols().len(), 1);
        assert_eq!(symdef.symbols()[0].uuid(), uuid("87654321-4321-4321-4321-ba0987654321"));
    }

    #[test]
    fn test_position_name_optional() {
        let xml = r#"<Position uuid="11111111-2222-3333-4444-555555555555"/>"#;
        let pos: Position = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(pos.uuid(), uuid("11111111-2222-3333-4444-555555555555"));
        assert_eq!(pos.name(), "");
    }

    #[test]
    fn test_mapping_definition_scale_handeling_optional() {
        let xml = r#"
            <MappingDefinition uuid="bbbbbbbb-cccc-dddd-eeee-ffffffffffff" name="Mapping1">
                <SizeX>100</SizeX>
                <SizeY>200</SizeY>
                <Source linkedGeometry="geom" type="NDI">stream</Source>
            </MappingDefinition>
        "#;
        let mapping: MappingDefinition = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(mapping.scale_handeling(), ScaleHandeling::ScaleKeepRatio); // default
    }

    #[test]
    fn test_mapping_definition_scale_handeling_variants() {
        let xml = r#"
            <MappingDefinition uuid="bbbbbbbb-cccc-dddd-eeee-ffffffffffff" name="Mapping1">
                <SizeX>100</SizeX>
                <SizeY>200</SizeY>
                <Source linkedGeometry="geom" type="NDI">stream</Source>
                <ScaleHandeling>ScaleIgnoreRatio</ScaleHandeling>
            </MappingDefinition>
        "#;
        let mapping: MappingDefinition = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(mapping.scale_handeling(), ScaleHandeling::ScaleIgnoreRatio);

        let xml2 = r#"
            <MappingDefinition uuid="bbbbbbbb-cccc-dddd-eeee-ffffffffffff" name="Mapping1">
                <SizeX>100</SizeX>
                <SizeY>200</SizeY>
                <Source linkedGeometry="geom" type="NDI">stream</Source>
                <ScaleHandeling>KeepSizeCenter</ScaleHandeling>
            </MappingDefinition>
        "#;
        let mapping2: MappingDefinition = quick_xml::de::from_str(xml2).unwrap();
        assert_eq!(mapping2.scale_handeling(), ScaleHandeling::KeepSizeCenter);
    }

    #[test]
    fn test_geometry3d_matrix_optional() {
        let xml = r#"<Geometry3D fileName="geom.obj"/>"#;
        let geom: Geometry3D = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(geom.file_name().as_str(), "geom.obj");
        assert!(geom.matrix().is_none());
    }

    #[test]
    fn test_symbol_matrix_optional() {
        let xml = r#"<Symbol uuid="11111111-2222-3333-4444-555555555555" symdef="aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"/>"#;
        let sym: Symbol = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(sym.uuid(), uuid("11111111-2222-3333-4444-555555555555"));
        assert_eq!(sym.symdef(), uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"));
        assert!(sym.matrix().is_none());
    }
}
