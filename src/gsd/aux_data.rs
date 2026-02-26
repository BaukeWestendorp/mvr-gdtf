use std::str::FromStr as _;
#[cfg(not(tarpaulin_include))]
use std::{fmt, ops, str};

use facet_xml as xml;
use uuid::Uuid;

use crate::{FileName, Matrix4x3};

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct AuxData {
    #[facet(rename = "Symdef")]
    pub(crate) symdefs: Vec<Symdef>,
    #[facet(rename = "Position")]
    pub(crate) positions: Vec<Position>,
    #[facet(rename = "MappingDefinition")]
    pub(crate) mapping_definitions: Vec<MappingDefinition>,
    #[facet(rename = "Class")]
    pub(crate) class: Option<Class>,
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
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
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

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Position {
    #[facet(xml::attribute, rename = "uuid")]
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
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

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Symdef {
    #[facet(xml::attribute, rename = "uuid")]
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    pub(crate) name: String,

    #[facet(rename = "ChildList", default)]
    pub(crate) child_list: SymdefChildList,
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct SymdefChildList {
    #[facet(rename = "Geometry3D")]
    pub(crate) geometry3ds: Vec<Geometry3D>,
    #[facet(rename = "Symbol")]
    pub(crate) symbols: Vec<Symbol>,
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
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    pub(crate) name: String,

    #[facet(rename = "SizeX")]
    pub(crate) size_x: SizeX,
    #[facet(rename = "SizeY")]
    pub(crate) size_y: SizeY,

    // FIXME: I can't seem to figure out how to directly parse this enum
    // using facet for some reason...
    #[facet(rename = "ScaleHandeling", default = "ScaleKeepRatio")]
    pub(crate) scale_handeling: String,

    #[facet(rename = "Source")]
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
    pub(crate) linked_geometry: String,
    #[facet(xml::attribute, rename = "type")]
    pub(crate) type_: SourceType,

    #[facet(xml::text)]
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

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum SourceType {
    Ndi,
    File,
    Citp,
    CaptureDevice,
}

#[cfg(not(tarpaulin_include))]
impl str::FromStr for SourceType {
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
    pub(crate) file_name: FileName,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(rename = "Matrix")]
    pub(crate) matrix: Option<String>,
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
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "symdef", default = "")]
    pub(crate) symdef: Uuid,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(rename = "Matrix")]
    pub(crate) matrix: Option<String>,
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

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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

#[derive(facet::Facet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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
