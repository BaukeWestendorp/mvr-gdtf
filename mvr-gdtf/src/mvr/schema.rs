use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AuxData {
    #[serde(default, rename = "Class")]
    pub class: ::std::vec::Vec<BasicChildListAttribute>,
    #[serde(default, rename = "Symdef")]
    pub symdef: ::std::vec::Vec<Symdef>,
    #[serde(default, rename = "Position")]
    pub position: ::std::vec::Vec<BasicChildListAttribute>,
    #[serde(default, rename = "MappingDefinition")]
    pub mapping_definition: ::std::vec::Vec<MappingDefinition>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(default = "Address::default_break_", rename = "@break")]
    pub break_: ::core::primitive::i32,
    #[serde(rename = "$text")]
    pub content: ::core::primitive::i32,
}
impl Address {
    #[must_use]
    pub fn default_break_() -> ::core::primitive::i32 {
        0i32
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Addresses {
    #[serde(default, rename = "Address")]
    pub address: ::std::vec::Vec<Address>,
    #[serde(default, rename = "Network")]
    pub network: ::std::vec::Vec<Network>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Alignment {
    #[serde(default, rename = "@geometry")]
    pub geometry: ::core::option::Option<::std::string::String>,
    #[serde(default = "Alignment::default_up", rename = "@up")]
    pub up: ::std::string::String,
    #[serde(default = "Alignment::default_direction", rename = "@direction")]
    pub direction: ::std::string::String,
}
impl Alignment {
    #[must_use]
    pub fn default_up() -> ::std::string::String {
        ::std::string::String::from("0,0,1")
    }
    #[must_use]
    pub fn default_direction() -> ::std::string::String {
        ::std::string::String::from("0,0,-1")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Alignments {
    #[serde(default, rename = "Alignment")]
    pub alignment: ::std::vec::Vec<Alignment>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BasicChildListAttribute {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "BasicChildListAttribute::default_name", rename = "@name")]
    pub name: ::std::string::String,
}
impl BasicChildListAttribute {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChildList {
    #[serde(default, rename = "$value")]
    pub content: ::std::vec::Vec<ChildListContent>,
}
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "@own")]
    pub own: ::std::string::String,
    #[serde(rename = "@other")]
    pub other: ::std::string::String,
    #[serde(rename = "@toObject")]
    pub to_object: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Connections {
    #[serde(default, rename = "Connection")]
    pub connection: ::std::vec::Vec<Connection>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomCommands {
    #[serde(default, rename = "CustomCommand")]
    pub custom_command: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "@provider")]
    pub provider: ::std::string::String,
    #[serde(default = "Data::default_ver", rename = "@ver")]
    pub ver: ::std::string::String,
}
impl Data {
    #[must_use]
    pub fn default_ver() -> ::std::string::String {
        ::std::string::String::from("1")
    }
}
pub type FileName = ::std::string::String;
#[derive(Debug, Deserialize, Serialize)]
pub struct Fixture {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Fixture::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "Fixture::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Focus")]
    pub focus: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "DMXInvertPan")]
    pub dmx_invert_pan: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "DMXInvertTilt")]
    pub dmx_invert_tilt: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Position")]
    pub position: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Function")]
    pub function: ::core::option::Option<::std::string::String>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: ::std::string::String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(rename = "UnitNumber")]
    pub unit_number: ::core::primitive::i32,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Protocols")]
    pub protocols: ::core::option::Option<Protocols>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(default, rename = "Color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "Mappings")]
    pub mappings: ::core::option::Option<Mappings>,
    #[serde(default, rename = "Gobo")]
    pub gobo: ::core::option::Option<Gobo>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
}
impl Fixture {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FocusPoint {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "FocusPoint::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
}
impl FocusPoint {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
pub type GeneralSceneDescription = GeneralSceneDescriptionElementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralSceneDescriptionElementType {
    #[serde(rename = "@verMajor")]
    pub ver_major: ::core::primitive::i32,
    #[serde(rename = "@verMinor")]
    pub ver_minor: ::core::primitive::i32,
    #[serde(default, rename = "@provider")]
    pub provider: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@providerVersion")]
    pub provider_version: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "UserData")]
    pub user_data: ::core::option::Option<UserData>,
    #[serde(rename = "Scene")]
    pub scene: Scene,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometries {
    #[serde(default, rename = "Geometry3D")]
    pub geometry_3d: ::std::vec::Vec<Geometry3D>,
    #[serde(default, rename = "Symbol")]
    pub symbol: ::std::vec::Vec<Symbol>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry3D {
    #[serde(rename = "@fileName")]
    pub file_name: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Gobo {
    #[serde(default = "Gobo::default_rotation", rename = "@rotation")]
    pub rotation: ::core::primitive::f32,
}
impl Gobo {
    #[must_use]
    pub fn default_rotation() -> ::core::primitive::f32 {
        0f32
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupObject {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "GroupObject::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(rename = "ChildList")]
    pub child_list: ::std::boxed::Box<ChildList>,
}
impl GroupObject {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
pub type Ipv4Adress = ::std::string::String;
pub type Ipv6Adress = ::std::string::String;
#[derive(Debug, Deserialize, Serialize)]
pub struct Layer {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Layer::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<ChildList>,
}
impl Layer {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Layers {
    #[serde(default, rename = "Layer")]
    pub layer: ::std::vec::Vec<Layer>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(rename = "@linkedDef")]
    pub linked_def: ::std::string::String,
    #[serde(default, rename = "ux")]
    pub ux: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "uy")]
    pub uy: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "ox")]
    pub ox: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "oy")]
    pub oy: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "rz")]
    pub rz: ::core::option::Option<::core::primitive::f32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MappingDefinition {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "MappingDefinition::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(rename = "SizeX")]
    pub size_x: ::core::primitive::i32,
    #[serde(rename = "SizeY")]
    pub size_y: ::core::primitive::i32,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: ::core::option::Option<ScaleHandeling>,
}
impl MappingDefinition {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mappings {
    #[serde(default, rename = "Mapping")]
    pub mapping: ::std::vec::Vec<Mapping>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    #[serde(rename = "@geometry")]
    pub geometry: ::std::string::String,
    #[serde(default, rename = "@ipv4")]
    pub ipv_4: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@subnetmask")]
    pub subnetmask: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@ipv6")]
    pub ipv_6: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@dhcp")]
    pub dhcp: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@hostname")]
    pub hostname: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrite {
    #[serde(rename = "@universal")]
    pub universal: ::std::string::String,
    #[serde(default = "Overwrite::default_target", rename = "@target")]
    pub target: ::std::string::String,
}
impl Overwrite {
    #[must_use]
    pub fn default_target() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Overwrites {
    #[serde(default, rename = "Overwrite")]
    pub overwrite: ::std::vec::Vec<Overwrite>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projection {
    #[serde(default, rename = "Source")]
    pub source: ::std::vec::Vec<Source>,
    #[serde(default, rename = "ScaleHandeling")]
    pub scale_handeling: ::std::vec::Vec<ScaleHandeling>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projections {
    #[serde(default, rename = "Projection")]
    pub projection: ::std::vec::Vec<Projection>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Projector {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Projector::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "Projector::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "Projections")]
    pub projections: Projections,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: ::std::string::String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
}
impl Projector {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Protocol {
    #[serde(default = "Protocol::default_geometry", rename = "@geometry")]
    pub geometry: ::std::string::String,
    #[serde(default = "Protocol::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "Protocol::default_type_", rename = "@type")]
    pub type_: ::std::string::String,
    #[serde(default = "Protocol::default_version", rename = "@version")]
    pub version: ::std::string::String,
    #[serde(default, rename = "@transmission")]
    pub transmission: ::core::option::Option<TransmissionEnum>,
}
impl Protocol {
    #[must_use]
    pub fn default_geometry() -> ::std::string::String {
        ::std::string::String::from("NetworkInOut_1")
    }
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_type_() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_version() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Protocols {
    #[serde(default, rename = "Protocol")]
    pub protocol: ::std::vec::Vec<Protocol>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ScaleHandeling {
    #[serde(default = "ScaleHandeling::default_enum_", rename = "@Enum")]
    pub enum_: Scaleenum,
}
impl ScaleHandeling {
    #[must_use]
    pub fn default_enum_() -> Scaleenum {
        Scaleenum::ScaleKeepRatio
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    #[serde(default, rename = "AUXData")]
    pub aux_data: ::core::option::Option<AuxData>,
    #[serde(rename = "Layers")]
    pub layers: Layers,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SceneObject {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "SceneObject::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "SceneObject::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(default, rename = "FixtureID")]
    pub fixture_id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
}
impl SceneObject {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "@linkedGeometry")]
    pub linked_geometry: ::std::string::String,
    #[serde(rename = "@type")]
    pub type_: SourceEnum,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SourceEnum {
    #[serde(rename = "NDI")]
    Ndi,
    #[serde(rename = "File")]
    File,
    #[serde(rename = "CITP")]
    Citp,
    #[serde(rename = "CaptureDevice")]
    CaptureDevice,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Sources {
    #[serde(default, rename = "Source")]
    pub source: ::std::vec::Vec<Source>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Support {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Support::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "Support::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Position")]
    pub position: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: ::core::option::Option<::std::string::String>,
    #[serde(rename = "ChainLength")]
    pub chain_length: ::core::primitive::f32,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: ::std::string::String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
}
impl Support {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(rename = "@symdef")]
    pub symdef: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Symdef {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Symdef::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(rename = "ChildList")]
    pub child_list: SymdefChildList,
}
impl Symdef {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SymdefChildList {
    #[serde(default, rename = "Geometry3D")]
    pub geometry_3d: ::std::vec::Vec<Geometry3D>,
    #[serde(default, rename = "Symbol")]
    pub symbol: ::std::vec::Vec<Symbol>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum TransmissionEnum {
    #[serde(rename = "Unicast")]
    Unicast,
    #[serde(rename = "Multicast")]
    Multicast,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Anycast")]
    Anycast,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Truss {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "Truss::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "Truss::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Position")]
    pub position: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Function")]
    pub function: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(default, rename = "ChildPosition")]
    pub child_position: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: ::std::string::String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
}
impl Truss {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    #[serde(default, rename = "Data")]
    pub data: ::std::vec::Vec<Data>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct VideoScreen {
    #[serde(rename = "@uuid")]
    pub uuid: ::std::string::String,
    #[serde(default = "VideoScreen::default_name", rename = "@name")]
    pub name: ::std::string::String,
    #[serde(default = "VideoScreen::default_multipatch", rename = "@multipatch")]
    pub multipatch: ::std::string::String,
    #[serde(default, rename = "Matrix")]
    pub matrix: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Classing")]
    pub classing: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(default, rename = "Sources")]
    pub sources: ::core::option::Option<Sources>,
    #[serde(default, rename = "Function")]
    pub function: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFSpec")]
    pub gdtf_spec: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "GDTFMode")]
    pub gdtf_mode: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CastShadow")]
    pub cast_shadow: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, rename = "Addresses")]
    pub addresses: ::core::option::Option<Addresses>,
    #[serde(default, rename = "Alignments")]
    pub alignments: ::core::option::Option<Alignments>,
    #[serde(default, rename = "CustomCommands")]
    pub custom_commands: ::core::option::Option<CustomCommands>,
    #[serde(default, rename = "Overwrites")]
    pub overwrites: ::core::option::Option<Overwrites>,
    #[serde(default, rename = "Connections")]
    pub connections: ::core::option::Option<Connections>,
    #[serde(default, rename = "ChildList")]
    pub child_list: ::core::option::Option<::std::boxed::Box<ChildList>>,
    #[serde(rename = "FixtureID")]
    pub fixture_id: ::std::string::String,
    #[serde(default, rename = "FixtureIDNumeric")]
    pub fixture_id_numeric: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "FixtureTypeId")]
    pub fixture_type_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "UnitNumber")]
    pub unit_number: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomIdType")]
    pub custom_id_type: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "CustomId")]
    pub custom_id: ::core::option::Option<::core::primitive::i32>,
}
impl VideoScreen {
    #[must_use]
    pub fn default_name() -> ::std::string::String {
        ::std::string::String::from("")
    }
    #[must_use]
    pub fn default_multipatch() -> ::std::string::String {
        ::std::string::String::from("")
    }
}
pub type Ciecolortype = ::std::string::String;
pub type Guidtype = ::std::string::String;
pub type Matrixtype = ::std::string::String;
pub type Positiveinteger = ::core::primitive::i32;
#[derive(Debug, Deserialize, Serialize)]
pub enum Scaleenum {
    #[serde(rename = "ScaleKeepRatio")]
    ScaleKeepRatio,
    #[serde(rename = "ScaleIgnoreRatio")]
    ScaleIgnoreRatio,
    #[serde(rename = "KeepSizeCenter")]
    KeepSizeCenter,
}
pub mod xs {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Entities(pub ::std::vec::Vec<::std::string::String>);
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Entity(pub ::std::vec::Vec<::std::string::String>);
    pub type Id = ::std::string::String;
    pub type Idref = ::std::string::String;
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Idrefs(pub ::std::vec::Vec<::std::string::String>);
    pub type NcName = ::std::string::String;
    pub type Nmtoken = ::std::string::String;
    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Nmtokens(pub ::std::vec::Vec<::std::string::String>);
    pub type Notation = ::std::string::String;
    pub type Name = ::std::string::String;
    pub type QName = ::std::string::String;
    pub type AnySimpleType = ::std::string::String;
    pub type AnyUri = ::std::string::String;
    pub type Base64Binary = ::std::string::String;
    pub type Boolean = ::core::primitive::bool;
    pub type Byte = ::core::primitive::i8;
    pub type Date = ::std::string::String;
    pub type DateTime = ::std::string::String;
    pub type Decimal = ::core::primitive::f64;
    pub type Double = ::core::primitive::f64;
    pub type Duration = ::std::string::String;
    pub type Float = ::core::primitive::f32;
    pub type GDay = ::std::string::String;
    pub type GMonth = ::std::string::String;
    pub type GMonthDay = ::std::string::String;
    pub type GYear = ::std::string::String;
    pub type GYearMonth = ::std::string::String;
    pub type HexBinary = ::std::string::String;
    pub type Int = ::core::primitive::i32;
    pub type Integer = ::core::primitive::i32;
    pub type Language = ::std::string::String;
    pub type Long = ::core::primitive::i64;
    pub type NegativeInteger = ::core::primitive::isize;
    pub type NonNegativeInteger = ::core::primitive::usize;
    pub type NonPositiveInteger = ::core::primitive::isize;
    pub type NormalizedString = ::std::string::String;
    pub type PositiveInteger = ::core::primitive::usize;
    pub type Short = ::core::primitive::i16;
    pub type String = ::std::string::String;
    pub type Time = ::std::string::String;
    pub type Token = ::std::string::String;
    pub type UnsignedByte = ::core::primitive::u8;
    pub type UnsignedInt = ::core::primitive::u32;
    pub type UnsignedLong = ::core::primitive::u64;
    pub type UnsignedShort = ::core::primitive::u16;
}
