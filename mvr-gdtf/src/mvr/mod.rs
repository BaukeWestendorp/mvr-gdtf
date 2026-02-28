use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::{self, Deref, DerefMut};

use uuid::Uuid;

mod values;

pub use values::*;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuxData {
    #[serde(default, rename = "Class")]
    pub class: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "Symdef")]
    pub symdef: Vec<Symdef>,
    #[serde(default, rename = "Position")]
    pub position: Vec<BasicChildListAttribute>,
    #[serde(default, rename = "MappingDefinition")]
    pub mapping_definition: Vec<MappingDefinition>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Address {
    #[serde(default = "Address::default_break", rename = "@break")]
    pub break_: u32,
    #[serde(rename = "$text")]
    pub absolute: u32,
}

impl Address {
    pub fn default_break() -> u32 {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Addresses {
    #[serde(default, rename = "Address")]
    pub address: Vec<Address>,
    #[serde(default, rename = "Network")]
    pub network: Vec<Network>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Alignment {
    #[serde(default, rename = "@geometry")]
    pub geometry: Option<String>,
    #[serde(default = "Alignment::default_up", rename = "@up")]
    pub up: String,
    #[serde(default = "Alignment::default_direction", rename = "@direction")]
    pub direction: String,
}

impl Alignment {
    pub fn default_up() -> String {
        String::from("0,0,1")
    }

    pub fn default_direction() -> String {
        String::from("0,0,-1")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Alignments {
    #[serde(default, rename = "Alignment")]
    alignments: Vec<Alignment>,
}

impl Deref for Alignments {
    type Target = Vec<Alignment>;

    fn deref(&self) -> &Self::Target {
        &self.alignments
    }
}

impl DerefMut for Alignments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.alignments
    }
}

impl IntoIterator for Alignments {
    type Item = Alignment;
    type IntoIter = std::vec::IntoIter<Alignment>;

    fn into_iter(self) -> Self::IntoIter {
        self.alignments.into_iter()
    }
}

impl<'a> IntoIterator for &'a Alignments {
    type Item = &'a Alignment;
    type IntoIter = std::slice::Iter<'a, Alignment>;

    fn into_iter(self) -> Self::IntoIter {
        self.alignments.iter()
    }
}

impl<'a> IntoIterator for &'a mut Alignments {
    type Item = &'a mut Alignment;
    type IntoIter = std::slice::IterMut<'a, Alignment>;

    fn into_iter(self) -> Self::IntoIter {
        self.alignments.iter_mut()
    }
}

impl From<Vec<Alignment>> for Alignments {
    fn from(vec: Vec<Alignment>) -> Self {
        Alignments { alignments: vec }
    }
}

impl From<Alignments> for Vec<Alignment> {
    fn from(a: Alignments) -> Self {
        a.alignments
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BasicChildListAttribute {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "BasicChildListAttribute::default_name", rename = "@name")]
    pub name: String,
}

impl BasicChildListAttribute {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChildList {
    #[serde(default, rename = "$value")]
    pub contents: Vec<ChildListContent>,
}

impl Deref for ChildList {
    type Target = Vec<ChildListContent>;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl DerefMut for ChildList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contents
    }
}

impl IntoIterator for ChildList {
    type Item = ChildListContent;
    type IntoIter = std::vec::IntoIter<ChildListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl<'a> IntoIterator for &'a ChildList {
    type Item = &'a ChildListContent;
    type IntoIter = std::slice::Iter<'a, ChildListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl<'a> IntoIterator for &'a mut ChildList {
    type Item = &'a mut ChildListContent;
    type IntoIter = std::slice::IterMut<'a, ChildListContent>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter_mut()
    }
}

impl From<Vec<ChildListContent>> for ChildList {
    fn from(vec: Vec<ChildListContent>) -> Self {
        ChildList { contents: vec }
    }
}

impl From<ChildList> for Vec<ChildListContent> {
    fn from(c: ChildList) -> Self {
        c.contents
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChildListContent {
    #[serde(rename = "SceneObject")]
    SceneObject(SceneObject),
    #[serde(rename = "GroupObject")]
    GroupObject(GroupObject),
    #[serde(rename = "FocusPoint")]
    FocusPoint(FocusPoint),
    #[serde(rename = "Fixture")]
    Fixture(Fixture),
    #[serde(rename = "Support")]
    Support(Support),
    #[serde(rename = "Truss")]
    Truss(Truss),
    #[serde(rename = "VideoScreen")]
    VideoScreen(VideoScreen),
    #[serde(rename = "Projector")]
    Projector(Projector),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connection {
    #[serde(rename = "@own")]
    pub own: String,
    #[serde(rename = "@other")]
    pub other: String,
    #[serde(rename = "@toObject")]
    pub to_object: Uuid,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connections {
    #[serde(default, rename = "Connection")]
    pub connections: Vec<Connection>,
}

impl Deref for Connections {
    type Target = Vec<Connection>;

    fn deref(&self) -> &Self::Target {
        &self.connections
    }
}

impl DerefMut for Connections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connections
    }
}

impl IntoIterator for Connections {
    type Item = Connection;
    type IntoIter = std::vec::IntoIter<Connection>;

    fn into_iter(self) -> Self::IntoIter {
        self.connections.into_iter()
    }
}

impl<'a> IntoIterator for &'a Connections {
    type Item = &'a Connection;
    type IntoIter = std::slice::Iter<'a, Connection>;

    fn into_iter(self) -> Self::IntoIter {
        self.connections.iter()
    }
}

impl<'a> IntoIterator for &'a mut Connections {
    type Item = &'a mut Connection;
    type IntoIter = std::slice::IterMut<'a, Connection>;

    fn into_iter(self) -> Self::IntoIter {
        self.connections.iter_mut()
    }
}

impl From<Vec<Connection>> for Connections {
    fn from(vec: Vec<Connection>) -> Self {
        Connections { connections: vec }
    }
}

impl From<Connections> for Vec<Connection> {
    fn from(c: Connections) -> Self {
        c.connections
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CustomCommands {
    #[serde(default, rename = "CustomCommand")]
    commands: Vec<String>,
}

impl Deref for CustomCommands {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl DerefMut for CustomCommands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}

impl IntoIterator for CustomCommands {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

impl<'a> IntoIterator for &'a CustomCommands {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter()
    }
}

impl<'a> IntoIterator for &'a mut CustomCommands {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.iter_mut()
    }
}

impl From<Vec<String>> for CustomCommands {
    fn from(vec: Vec<String>) -> Self {
        CustomCommands { commands: vec }
    }
}

impl From<CustomCommands> for Vec<String> {
    fn from(c: CustomCommands) -> Self {
        c.commands
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Data {
    #[serde(rename = "@provider")]
    pub provider: String,
    #[serde(default = "Data::default_ver", rename = "@ver")]
    pub ver: String,
}

impl Data {
    pub fn default_ver() -> String {
        String::from("1")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Fixture {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Fixture::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Fixture::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<FileName>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "Focus")]
    pub focus: Option<Uuid>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "DMXInvertPan")]
    pub dmx_invert_pan: Option<bool>,
    #[serde(default, rename = "DMXInvertTilt")]
    pub dmx_invert_tilt: Option<bool>,
    #[serde(default, rename = "Position")]
    pub position: Option<Uuid>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Protocols")]
    pub protocols: Protocols,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(default, rename = "Color")]
    pub color: Option<CieColor>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "Mappings")]
    pub mappings: Mappings,
    #[serde(default, rename = "Gobo")]
    pub gobo: Option<Gobo>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
}

impl Fixture {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FocusPoint {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "FocusPoint::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
}

impl FocusPoint {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneralSceneDescription {
    #[serde(rename = "@verMajor")]
    pub ver_major: u32,
    #[serde(rename = "@verMinor")]
    pub ver_minor: u32,
    #[serde(default, rename = "@provider")]
    pub provider: Option<String>,
    #[serde(default, rename = "@providerVersion")]
    pub provider_version: Option<String>,
    #[serde(default, rename = "UserData")]
    pub user_data: Option<UserData>,
    #[serde(rename = "Scene")]
    pub scene: Scene,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometries {
    #[serde(default, rename = "Geometry3D")]
    pub geometry_3d: Vec<Geometry3D>,
    #[serde(default, rename = "Symbol")]
    pub symbol: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometry3D {
    #[serde(rename = "@fileName")]
    pub file_name: FileName,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gobo {
    #[serde(default = "Gobo::default_rotation", rename = "@rotation")]
    pub rotation: f32,
}

impl Gobo {
    pub fn default_rotation() -> f32 {
        0f32
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GroupObject {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "GroupObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(rename = "ChildList")]
    pub child_list: Box<ChildList>,
}

impl GroupObject {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layer {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Layer::default_name", rename = "@name")]
    pub name: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ChildList,
}

impl Layer {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layers {
    #[serde(default, rename = "Layer")]
    layers: Vec<Layer>,
}

impl Deref for Layers {
    type Target = Vec<Layer>;

    fn deref(&self) -> &Self::Target {
        &self.layers
    }
}

impl DerefMut for Layers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layers
    }
}

impl IntoIterator for Layers {
    type Item = Layer;
    type IntoIter = std::vec::IntoIter<Layer>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Layers {
    type Item = &'a Layer;
    type IntoIter = std::slice::Iter<'a, Layer>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Layers {
    type Item = &'a mut Layer;
    type IntoIter = std::slice::IterMut<'a, Layer>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.iter_mut()
    }
}

impl From<Vec<Layer>> for Layers {
    fn from(vec: Vec<Layer>) -> Self {
        Layers { layers: vec }
    }
}

impl From<Layers> for Vec<Layer> {
    fn from(l: Layers) -> Self {
        l.layers
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mapping {
    #[serde(rename = "@linkedDef")]
    pub linked_def: Uuid,
    #[serde(default, rename = "ux")]
    pub ux: Option<i32>,
    #[serde(default, rename = "uy")]
    pub uy: Option<i32>,
    #[serde(default, rename = "ox")]
    pub ox: Option<i32>,
    #[serde(default, rename = "oy")]
    pub oy: Option<i32>,
    #[serde(default, rename = "rz")]
    pub rz: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MappingDefinition {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "MappingDefinition::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "SizeX")]
    pub size_x: i32,
    #[serde(rename = "SizeY")]
    pub size_y: i32,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Option<ScaleHandling>,
}

impl MappingDefinition {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mappings {
    #[serde(default, rename = "Mapping")]
    pub mapping: Vec<Mapping>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Network {
    #[serde(rename = "@geometry")]
    pub geometry: String,
    #[serde(default, rename = "@ipv4")]
    pub ipv4: Option<Ipv4Addr>,
    #[serde(default, rename = "@subnetmask")]
    pub subnetmask: Option<Ipv4Addr>,
    #[serde(default, rename = "@ipv6")]
    pub ipv6: Option<Ipv6Addr>,
    #[serde(default, rename = "@dhcp")]
    pub dhcp: Option<String>,
    #[serde(default, rename = "@hostname")]
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Overwrite {
    #[serde(rename = "@universal")]
    pub universal: String,
    #[serde(default = "Overwrite::default_target", rename = "@target")]
    pub target: String,
}

impl Overwrite {
    pub fn default_target() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Overwrites {
    #[serde(default, rename = "Overwrite")]
    pub overwrite: Vec<Overwrite>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projection {
    #[serde(default, rename = "Source")]
    pub source: Vec<Source>,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: Vec<ScaleHandling>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projections {
    #[serde(default, rename = "Projection")]
    pub projection: Vec<Projection>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Projector {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Projector::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Projector::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "Projections")]
    pub projections: Projections,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl Projector {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocol {
    #[serde(default = "Protocol::default_geometry", rename = "@geometry")]
    pub geometry: String,
    #[serde(default = "Protocol::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Protocol::default_type", rename = "@type")]
    pub type_: String,
    #[serde(default = "Protocol::default_version", rename = "@version")]
    pub version: String,
    #[serde(default, rename = "@transmission")]
    pub transmission: Option<TransmissionType>,
}

impl Protocol {
    pub fn default_geometry() -> String {
        String::from("NetworkInOut_1")
    }

    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_type() -> String {
        String::from("")
    }

    pub fn default_version() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    #[serde(default, rename = "Protocol")]
    pub protocol: Vec<Protocol>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Scene {
    #[serde(default, rename = "AUXData")]
    pub aux_data: Option<AuxData>,
    #[serde(rename = "Layers")]
    pub layers: Layers,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent, rename = "ScaleHandeling")]
pub struct ScaleHandling(#[serde(rename = "$text")] pub ScaleHandlingType);

impl ops::Deref for ScaleHandling {
    type Target = ScaleHandlingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ScaleHandling {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SceneObject {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "SceneObject::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "SceneObject::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: Option<String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
}

impl SceneObject {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Source {
    #[serde(rename = "@linkedGeometry")]
    pub linked_geometry: String,
    #[serde(rename = "@type")]
    pub type_: SourceType,
    #[serde(default, rename = "$text")]
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
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
pub struct Sources {
    #[serde(default, rename = "Source")]
    pub source: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Support {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Support::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Support::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(default, rename = "Position")]
    pub position: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(rename = "ChainLength")]
    pub chain_length: f32,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
}

impl Support {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Symbol {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(rename = "@symdef")]
    pub symdef: String,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Symdef {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Symdef::default_name", rename = "@name")]
    pub name: String,
    #[serde(rename = "ChildList")]
    pub child_list: Geometries,
}

impl Symdef {
    pub fn default_name() -> String {
        String::from("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TransmissionType {
    #[serde(rename = "Unicast")]
    Unicast,
    #[serde(rename = "Multicast")]
    Multicast,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Anycast")]
    Anycast,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Truss {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "Truss::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "Truss::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(default, rename = "Position")]
    pub position: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: Option<String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl Truss {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserData {
    #[serde(default, rename = "Data")]
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VideoScreen {
    #[serde(rename = "@uuid")]
    pub uuid: Uuid,
    #[serde(default = "VideoScreen::default_name", rename = "@name")]
    pub name: String,
    #[serde(default = "VideoScreen::default_multipatch", rename = "@multipatch")]
    pub multipatch: Option<Uuid>,
    #[serde(default, rename = "Matrix")]
    pub matrix: Option<Matrix>,
    #[serde(default, rename = "Classing")]
    pub classing: Option<Uuid>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Sources")]
    pub sources: Option<Sources>,
    #[serde(default, rename = "Function")]
    pub function: Option<String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: Option<String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: Option<String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: Option<bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: Addresses,
    #[serde(default, rename = "Alignments")]
    pub alignments: Alignments,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: CustomCommands,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: Overwrites,
    #[serde(default, rename = "Connections")]
    pub connections: Connections,
    #[serde(default, rename = "ChildList")]
    pub child_list: Box<ChildList>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: Option<u32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: Option<u32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: Option<u32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: Option<i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: Option<i32>,
}

impl VideoScreen {
    pub fn default_name() -> String {
        String::from("")
    }

    pub fn default_multipatch() -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ScaleHandlingType {
    #[default]
    #[serde(rename = "ScaleKeepRatio")]
    ScaleKeepRatio,
    #[serde(rename = "ScaleIgnoreRatio")]
    ScaleIgnoreRatio,
    #[serde(rename = "KeepSizeCenter")]
    KeepSizeCenter,
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::*;

    #[derive(Debug, serde::Deserialize)]
    struct Wrapper {
        #[serde(rename = "ScaleHandeling")]
        scale_handling: ScaleHandling,
    }

    #[test]
    fn test_deserialize_scale_keep_ratio() {
        let xml = r#"<Wrapper><ScaleHandeling>ScaleKeepRatio</ScaleHandeling></Wrapper>"#;
        let val: Wrapper = from_str(xml).unwrap();
        assert_eq!(val.scale_handling.0, ScaleHandlingType::ScaleKeepRatio);
    }

    #[test]
    fn test_deserialize_scale_ignore_ratio() {
        let xml = r#"<Wrapper><ScaleHandeling>ScaleIgnoreRatio</ScaleHandeling></Wrapper>"#;
        let val: Wrapper = from_str(xml).unwrap();
        assert_eq!(val.scale_handling.0, ScaleHandlingType::ScaleIgnoreRatio);
    }

    #[test]
    fn test_deserialize_keep_size_center() {
        let xml = r#"<Wrapper><ScaleHandeling>KeepSizeCenter</ScaleHandeling></Wrapper>"#;
        let val: Wrapper = from_str(xml).unwrap();
        assert_eq!(val.scale_handling.0, ScaleHandlingType::KeepSizeCenter);
    }

    #[test]
    fn test_default_is_scale_keep_ratio() {
        let val: ScaleHandling = Default::default();
        assert_eq!(val.0, ScaleHandlingType::ScaleKeepRatio);
    }
}
