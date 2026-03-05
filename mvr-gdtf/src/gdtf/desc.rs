use std::{
    collections::HashSet,
    fmt,
    str::{self, FromStr as _},
};

use uuid::Uuid;

use crate::{
    CieColor, DmxAddress, DmxBreak, DmxOffset, DmxValue, FeatureType, Matrix4x4, Name, Node,
    PhysicalValue, Vector3, deserialize_option_from_string_none, serialize_option_as_string_none,
};

/// A data version string formatted as `major.minor` where each is a u8.
///
/// Spec Table 2: `UInt.UInt` (each 1 byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataVersion {
    pub major: u8,
    pub minor: u8,
}

impl str::FromStr for DataVersion {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (maj_s, min_s) =
            s.split_once('.').ok_or_else(|| crate::Error::InvalidDataVersion(s.to_owned()))?;

        let major: u8 =
            maj_s.parse().map_err(|_| crate::Error::InvalidDataVersion(s.to_owned()))?;
        let minor: u8 =
            min_s.parse().map_err(|_| crate::Error::InvalidDataVersion(s.to_owned()))?;

        Ok(DataVersion { major, minor })
    }
}

impl fmt::Display for DataVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl serde::Serialize for DataVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for DataVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<DataVersion>().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActivationGroup {
    #[serde(rename = "@Name")]
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActivationGroups {
    #[serde(default, rename = "ActivationGroup")]
    pub activation_group: Vec<ActivationGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AdditionalColorSpaces {
    #[serde(default, rename = "ColorSpace")]
    pub color_spaces: Vec<ColorSpace>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnimationSystem {
    #[serde(
        rename = "@P1",
        deserialize_with = "deserialize_two_array",
        serialize_with = "serialize_two_array"
    )]
    pub p1: (f32, f32),
    #[serde(
        rename = "@P2",
        deserialize_with = "deserialize_two_array",
        serialize_with = "serialize_two_array"
    )]
    pub p2: (f32, f32),
    #[serde(
        rename = "@P3",
        deserialize_with = "deserialize_two_array",
        serialize_with = "serialize_two_array"
    )]
    pub p3: (f32, f32),
    #[serde(rename = "@Radius")]
    pub radius: f32,
}

fn deserialize_two_array<'de, D>(deserializer: D) -> Result<(f32, f32), D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize as _;
    let s: String = String::deserialize(deserializer)?;
    let mut parts = s.split(',');
    let first = parts
        .next()
        .ok_or_else(|| serde::de::Error::custom("Missing first value in tuple"))?
        .parse::<f32>()
        .map_err(|_| serde::de::Error::custom("Failed to parse first value as f32"))?;
    let second = parts
        .next()
        .ok_or_else(|| serde::de::Error::custom("Missing second value in tuple"))?
        .parse::<f32>()
        .map_err(|_| serde::de::Error::custom("Failed to parse second value as f32"))?;
    Ok((first, second))
}

fn serialize_two_array<S>(tuple: &(f32, f32), serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{},{}", tuple.0, tuple.1))
}
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArtNet {
    #[serde(default, rename = "Map")]
    pub maps: Vec<Map>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attribute {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(rename = "@Pretty")]
    pub pretty: String,
    #[serde(default, rename = "@ActivationGroup")]
    pub activation_group: Option<String>,
    #[serde(rename = "@Feature")]
    pub feature: FeatureType,
    #[serde(default, rename = "@MainAttribute")]
    pub main_attribute: Option<Name>,
    #[serde(default, rename = "@PhysicalUnit")]
    pub physical_unit: PhysicalUnitType,
    #[serde(default, rename = "@Color")]
    pub color: Option<Vector3>,
    #[serde(default, rename = "SubPhysicalUnit")]
    pub sub_physical_unit: Vec<SubPhysicalUnit>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDefinitions {
    #[serde(default, rename = "ActivationGroups")]
    pub activation_groups: Option<ActivationGroups>,
    #[serde(rename = "FeatureGroups")]
    pub feature_groups: FeatureGroups,
    #[serde(rename = "Attributes")]
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attributes {
    #[serde(default, rename = "Attribute")]
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometry {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AnyGeometry {
    #[serde(rename = "Geometry")]
    Geometry(Geometry),
    #[serde(rename = "Axis")]
    Axis(Geometry),
    #[serde(rename = "FilterBeam")]
    FilterBeam(Geometry),
    #[serde(rename = "FilterColor")]
    FilterColor(Geometry),
    #[serde(rename = "FilterGobo")]
    FilterGobo(Geometry),
    #[serde(rename = "FilterShaper")]
    FilterShaper(Geometry),
    #[serde(rename = "Beam")]
    Beam(Beam),
    #[serde(rename = "MediaServerLayer")]
    MediaServerLayer(Geometry),
    #[serde(rename = "MediaServerCamera")]
    MediaServerCamera(Geometry),
    #[serde(rename = "MediaServerMaster")]
    MediaServerMaster(Geometry),
    #[serde(rename = "Display")]
    Display(Display),
    #[serde(rename = "Laser")]
    Laser(Laser),
    #[serde(rename = "GeometryReference")]
    GeometryReference(GeometryReference),
    #[serde(rename = "WiringObject")]
    WiringObject(WiringObject),
    #[serde(rename = "Inventory")]
    Inventory(Inventory),
    #[serde(rename = "Structure")]
    Structure(Structure),
    #[serde(rename = "Support")]
    Support(Support),
    #[serde(rename = "Magnet")]
    Magnet(Geometry),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Beam {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(default, rename = "@LampType")]
    pub lamp_type: Option<LampType>,
    #[serde(default, rename = "@PowerConsumption")]
    pub power_consumption: Option<f32>,
    #[serde(default, rename = "@LuminousFlux")]
    pub luminous_flux: Option<f32>,
    #[serde(default, rename = "@ColorTemperature")]
    pub color_temperature: Option<f32>,
    #[serde(default, rename = "@BeamAngle")]
    pub beam_angle: Option<f32>,
    #[serde(default, rename = "@FieldAngle")]
    pub field_angle: Option<f32>,
    #[serde(default, rename = "@ThrowRatio")]
    pub throw_ratio: Option<f32>,
    #[serde(default, rename = "@RectangleRatio")]
    pub rectangle_ratio: Option<f32>,
    #[serde(default, rename = "@BeamRadius")]
    pub beam_radius: Option<f32>,
    #[serde(default, rename = "@BeamType")]
    pub beam_type: Option<BeamType>,
    #[serde(default, rename = "@ColorRenderingIndex")]
    pub color_rendering_index: Option<u8>,
    #[serde(default, rename = "@EmitterSpectrum")]
    pub emitter_spectrum: Option<Node>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BeamType {
    #[serde(rename = "Wash")]
    Wash,
    #[serde(rename = "Spot")]
    Spot,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Rectangle")]
    Rectangle,
    #[serde(rename = "PC")]
    Pc,
    #[serde(rename = "Fresnel")]
    Fresnel,
    #[serde(rename = "Glow")]
    Glow,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Break {
    #[serde(default, rename = "@DMXOffset")]
    pub dmx_offset: DmxAddress,
    #[serde(default = "Break::default_dmx_break", rename = "@DMXBreak")]
    pub dmx_break: u8,
}

impl Break {
    pub fn default_dmx_break() -> u8 {
        1u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Ces {
    #[default]
    #[serde(rename = "CES01")]
    Ces01,
    #[serde(rename = "CES02")]
    Ces02,
    #[serde(rename = "CES03")]
    Ces03,
    #[serde(rename = "CES04")]
    Ces04,
    #[serde(rename = "CES05")]
    Ces05,
    #[serde(rename = "CES06")]
    Ces06,
    #[serde(rename = "CES07")]
    Ces07,
    #[serde(rename = "CES08")]
    Ces08,
    #[serde(rename = "CES09")]
    Ces09,
    #[serde(rename = "CES10")]
    Ces10,
    #[serde(rename = "CES11")]
    Ces11,
    #[serde(rename = "CES12")]
    Ces12,
    #[serde(rename = "CES13")]
    Ces13,
    #[serde(rename = "CES14")]
    Ces14,
    #[serde(rename = "CES15")]
    Ces15,
    #[serde(rename = "CES16")]
    Ces16,
    #[serde(rename = "CES17")]
    Ces17,
    #[serde(rename = "CES18")]
    Ces18,
    #[serde(rename = "CES19")]
    Ces19,
    #[serde(rename = "CES20")]
    Ces20,
    #[serde(rename = "CES21")]
    Ces21,
    #[serde(rename = "CES22")]
    Ces22,
    #[serde(rename = "CES23")]
    Ces23,
    #[serde(rename = "CES24")]
    Ces24,
    #[serde(rename = "CES25")]
    Ces25,
    #[serde(rename = "CES26")]
    Ces26,
    #[serde(rename = "CES27")]
    Ces27,
    #[serde(rename = "CES28")]
    Ces28,
    #[serde(rename = "CES29")]
    Ces29,
    #[serde(rename = "CES30")]
    Ces30,
    #[serde(rename = "CES31")]
    Ces31,
    #[serde(rename = "CES32")]
    Ces32,
    #[serde(rename = "CES33")]
    Ces33,
    #[serde(rename = "CES34")]
    Ces34,
    #[serde(rename = "CES35")]
    Ces35,
    #[serde(rename = "CES36")]
    Ces36,
    #[serde(rename = "CES37")]
    Ces37,
    #[serde(rename = "CES38")]
    Ces38,
    #[serde(rename = "CES39")]
    Ces39,
    #[serde(rename = "CES40")]
    Ces40,
    #[serde(rename = "CES41")]
    Ces41,
    #[serde(rename = "CES42")]
    Ces42,
    #[serde(rename = "CES43")]
    Ces43,
    #[serde(rename = "CES44")]
    Ces44,
    #[serde(rename = "CES45")]
    Ces45,
    #[serde(rename = "CES46")]
    Ces46,
    #[serde(rename = "CES47")]
    Ces47,
    #[serde(rename = "CES48")]
    Ces48,
    #[serde(rename = "CES49")]
    Ces49,
    #[serde(rename = "CES50")]
    Ces50,
    #[serde(rename = "CES51")]
    Ces51,
    #[serde(rename = "CES52")]
    Ces52,
    #[serde(rename = "CES53")]
    Ces53,
    #[serde(rename = "CES54")]
    Ces54,
    #[serde(rename = "CES55")]
    Ces55,
    #[serde(rename = "CES56")]
    Ces56,
    #[serde(rename = "CES57")]
    Ces57,
    #[serde(rename = "CES58")]
    Ces58,
    #[serde(rename = "CES59")]
    Ces59,
    #[serde(rename = "CES60")]
    Ces60,
    #[serde(rename = "CES61")]
    Ces61,
    #[serde(rename = "CES62")]
    Ces62,
    #[serde(rename = "CES63")]
    Ces63,
    #[serde(rename = "CES64")]
    Ces64,
    #[serde(rename = "CES65")]
    Ces65,
    #[serde(rename = "CES66")]
    Ces66,
    #[serde(rename = "CES67")]
    Ces67,
    #[serde(rename = "CES68")]
    Ces68,
    #[serde(rename = "CES69")]
    Ces69,
    #[serde(rename = "CES70")]
    Ces70,
    #[serde(rename = "CES71")]
    Ces71,
    #[serde(rename = "CES72")]
    Ces72,
    #[serde(rename = "CES73")]
    Ces73,
    #[serde(rename = "CES74")]
    Ces74,
    #[serde(rename = "CES75")]
    Ces75,
    #[serde(rename = "CES76")]
    Ces76,
    #[serde(rename = "CES77")]
    Ces77,
    #[serde(rename = "CES78")]
    Ces78,
    #[serde(rename = "CES79")]
    Ces79,
    #[serde(rename = "CES80")]
    Ces80,
    #[serde(rename = "CES81")]
    Ces81,
    #[serde(rename = "CES82")]
    Ces82,
    #[serde(rename = "CES83")]
    Ces83,
    #[serde(rename = "CES84")]
    Ces84,
    #[serde(rename = "CES85")]
    Ces85,
    #[serde(rename = "CES86")]
    Ces86,
    #[serde(rename = "CES87")]
    Ces87,
    #[serde(rename = "CES88")]
    Ces88,
    #[serde(rename = "CES89")]
    Ces89,
    #[serde(rename = "CES90")]
    Ces90,
    #[serde(rename = "CES91")]
    Ces91,
    #[serde(rename = "CES92")]
    Ces92,
    #[serde(rename = "CES93")]
    Ces93,
    #[serde(rename = "CES94")]
    Ces94,
    #[serde(rename = "CES95")]
    Ces95,
    #[serde(rename = "CES96")]
    Ces96,
    #[serde(rename = "CES97")]
    Ces97,
    #[serde(rename = "CES98")]
    Ces98,
    #[serde(rename = "CES99")]
    Ces99,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SAcn {
    #[serde(default, rename = "Map")]
    pub maps: Vec<Map>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PosiStageNet;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenSoundControl;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Citp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cri {
    #[serde(default, rename = "@CES")]
    pub ces: Ces,
    #[serde(default, rename = "@ColorRenderingIndex")]
    pub color_rendering_index: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CriGroup {
    #[serde(default, rename = "@ColorTemperature")]
    pub color_temperature: Option<f32>,
    #[serde(rename = "CRI")]
    pub cris: HashSet<Cri>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cris {
    #[serde(default, rename = "CRIGroup")]
    pub cri_groups: Vec<CriGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelFunction {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default = "ChannelFunction::default_attribute", rename = "@Attribute")]
    pub attribute: Name,
    #[serde(default, rename = "@OriginalAttribute")]
    pub original_attribute: String,
    #[serde(default, rename = "@DMXFrom")]
    pub dmx_from: DmxValue,
    #[serde(default, rename = "@Default")]
    pub default: DmxValue,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
    #[serde(default, rename = "@RealFade")]
    pub real_fade: Option<f32>,
    #[serde(default, rename = "@RealAcceleration")]
    pub real_acceleration: Option<f32>,
    #[serde(default, rename = "@Wheel")]
    pub wheel: Option<Name>,
    #[serde(default, rename = "@Emitter")]
    pub emitter: Option<Name>,
    #[serde(default, rename = "@Filter")]
    pub filter: Option<Name>,
    #[serde(default, rename = "@ColorSpace")]
    pub color_space: Option<Name>,
    #[serde(default, rename = "@Gammut")]
    pub gammut: Option<Name>,
    #[serde(default, rename = "@ModeMaster")]
    pub mode_master: Option<Node>,
    #[serde(default, rename = "@ModeFrom")]
    pub mode_from: DmxValue,
    #[serde(default, rename = "@ModeTo")]
    pub mode_to: DmxValue,
    #[serde(default, rename = "@DMXProfile")]
    pub dmx_profile: Option<Node>,
    #[serde(default, rename = "@Min")]
    pub min: Option<f32>,
    #[serde(default, rename = "@Max")]
    pub max: Option<f32>,
    #[serde(default, rename = "@CustomName")]
    pub custom_name: Option<String>,
    #[serde(default, rename = "ChannelSet")]
    pub channel_sets: Vec<ChannelSet>,
    #[serde(default, rename = "SubChannelSet")]
    pub sub_channel_sets: Vec<SubChannelSet>,
}

impl ChannelFunction {
    pub fn default_attribute() -> Name {
        Name::from_str("NoFeature").expect("NoFeature must be a valid Name")
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelSet {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "@DMXFrom")]
    pub dmx_from: DmxValue,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
    #[serde(default, rename = "@WheelSlotIndex")]
    pub wheel_slot_index: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ColorSpace {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "@Mode")]
    pub mode: ColorSpaceMode,
    #[serde(default, rename = "@Red")]
    pub red: Option<Vector3>,
    #[serde(default, rename = "@Green")]
    pub green: Option<Vector3>,
    #[serde(default, rename = "@Blue")]
    pub blue: Option<Vector3>,
    #[serde(default, rename = "@WhitePoint")]
    pub white_point: Option<Vector3>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ColorSpaceMode {
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "sRGB")]
    #[default]
    SRgb,
    #[serde(rename = "ProPhoto")]
    ProPhoto,
    #[serde(rename = "ANSI")]
    Ansi,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connector {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(rename = "@Type")]
    pub type_: Name,
    #[serde(default, rename = "@DMXBreak")]
    pub dmx_break: Option<u32>,
    #[serde(default, rename = "@Gender")]
    pub gender: Option<i32>,
    #[serde(default, rename = "@Length")]
    pub length: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Connectors {
    #[serde(default, rename = "Connector")]
    pub connectors: Vec<Connector>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CrossSectionType {
    #[serde(rename = "TrussFramework")]
    TrussFramework,
    #[serde(rename = "Tube")]
    Tube,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxChannel {
    #[serde(default, rename = "@DMXBreak")]
    pub dmx_break: DmxBreak,
    #[serde(
        default,
        rename = "@Offset",
        deserialize_with = "deserialize_option_from_string_none",
        serialize_with = "serialize_option_as_string_none"
    )]
    pub offset: Option<DmxOffset>,
    #[serde(default, rename = "@InitialFunction")]
    pub initial_function: Option<Node>,
    #[serde(
        default,
        rename = "@Highlight",
        deserialize_with = "deserialize_option_from_string_none",
        serialize_with = "serialize_option_as_string_none"
    )]
    pub highlight: Option<DmxValue>,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "LogicalChannel")]
    pub logical_channels: Vec<LogicalChannel>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxChannels {
    #[serde(default, rename = "DMXChannel")]
    pub dmx_channels: Vec<DmxChannel>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxMode {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "@Description")]
    pub description: Option<String>,
    #[serde(rename = "DMXChannels")]
    pub dmx_channels: DmxChannels,
    #[serde(default, rename = "Relations")]
    pub relations: Option<Relations>,
    #[serde(default, rename = "FTMacros")]
    pub ft_macros: Option<FtMacros>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxModes {
    #[serde(default, rename = "DMXMode")]
    pub dmx_modes: Vec<DmxMode>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxPersonality {
    #[serde(default, rename = "@Value")]
    pub value: Option<String>,
    #[serde(default, rename = "@DMXMode")]
    pub dmx_mode: Option<Name>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxProfile {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "Point")]
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DmxProfiles {
    #[serde(default, rename = "DMXProfile")]
    pub dmx_profiles: Vec<DmxProfile>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Display {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(default, rename = "@Texture")]
    pub texture: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Emitter {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Color")]
    pub color: Option<Vector3>,
    #[serde(default, rename = "@DominantWaveLength")]
    pub dominant_wave_length: Option<f32>,
    #[serde(default, rename = "@DiodePart")]
    pub diode_part: Option<String>,
    #[serde(default, rename = "Measurement")]
    pub measurements: Vec<EmitterMeasurement>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmitterMeasurement {
    #[serde(rename = "@Physical")]
    pub physical: PhysicalValue,
    #[serde(rename = "@LuminousIntensity")]
    pub luminous_intensity: f32,
    #[serde(default, rename = "@InterpolationTo")]
    pub interpolation_to: InterpolationTo,
    #[serde(default, rename = "@Transmission")]
    pub transmission: Option<f32>,
    #[serde(default, rename = "MeasurementPoint")]
    pub measurement_points: Vec<MeasurementPoint>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Emitters {
    #[serde(default, rename = "Emitter")]
    pub emitters: Vec<Emitter>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtMacro {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@ChannelFunction")]
    pub channel_function: Option<Node>,
    #[serde(default, rename = "MacroDMX")]
    pub macro_dmx: Option<MacroDmx>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtMacros {
    #[serde(default, rename = "FTMacro")]
    pub ft_macros: Vec<FtMacro>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtPresets;

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FtRdm {
    #[serde(default, rename = "@ManufacturerID")]
    pub manufacturer_id: Option<String>,
    #[serde(default, rename = "@DeviceModelID")]
    pub device_model_id: Option<String>,
    #[serde(default, rename = "SoftwareVersionId")]
    pub software_version_ids: Vec<SoftwareVersionId>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Facet {
    #[serde(default, rename = "@Color")]
    pub color: CieColor,
    #[serde(rename = "@Rotation")]
    // FIXME: Implement rotation struct.
    pub rotation: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FeatureGroup {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(rename = "@Pretty")]
    pub pretty: String,
    #[serde(default, rename = "Feature")]
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Feature {
    #[serde(rename = "@Name")]
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FeatureGroups {
    #[serde(default, rename = "FeatureGroup")]
    pub feature_group: Vec<FeatureGroup>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Filter {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Color")]
    pub color: Option<Vector3>,
    #[serde(default, rename = "FilterMeasurement")]
    pub measurements: Vec<FilterMeasurement>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FilterMeasurement {
    #[serde(rename = "@Physical")]
    pub physical: PhysicalValue,
    #[serde(rename = "@Transmission")]
    pub transmission: f32,
    #[serde(default, rename = "@InterpolationTo")]
    pub interpolation_to: InterpolationTo,
    #[serde(default, rename = "MeasurementPoint")]
    pub points: Vec<MeasurementPoint>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Filters {
    #[serde(default, rename = "Filter")]
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FixtureType {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@ShortName")]
    pub short_name: Option<String>,
    #[serde(default, rename = "@LongName")]
    pub long_name: Option<String>,
    #[serde(rename = "@Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "@Description")]
    pub description: String,
    #[serde(rename = "@FixtureTypeID")]
    pub fixture_type_id: Uuid,
    #[serde(default, rename = "@Thumbnail")]
    pub thumbnail: Option<String>,
    #[serde(default, rename = "@ThumbnailOffsetX")]
    pub thumbnail_offset_x: Option<i32>,
    #[serde(default, rename = "@ThumbnailOffsetY")]
    pub thumbnail_offset_y: Option<i32>,
    #[serde(default, rename = "@RefFT", deserialize_with = "deserialize_option_uuid")]
    pub ref_ft: Option<Uuid>,
    // FIXME: Serialize "Yes", "No"
    #[serde(
        default = "FixtureType::default_can_have_children",
        deserialize_with = "deserialize_can_have_children",
        rename = "@CanHaveChildren"
    )]
    pub can_have_children: bool,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,
    #[serde(default, rename = "Wheels")]
    pub wheels: Option<Wheels>,
    #[serde(default, rename = "PhysicalDescriptions")]
    pub physical_descriptions: Option<PhysicalDescriptions>,
    #[serde(default, rename = "Models")]
    pub models: Option<Models>,
    #[serde(rename = "Geometries")]
    pub geometries: Geometries,
    #[serde(rename = "DMXModes")]
    pub dmx_modes: DmxModes,
    #[serde(default, rename = "Revisions")]
    pub revisions: Option<Revisions>,
    #[serde(default, rename = "FTPresets")]
    pub ft_presets: Option<FtPresets>,
    #[serde(default, rename = "Protocols")]
    pub protocols: Option<Protocols>,
}

impl FixtureType {
    pub fn default_can_have_children() -> bool {
        true
    }
}

// Custom deserializer for Option<Uuid> that returns None if the string is empty or missing
fn deserialize_option_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize as _;
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref s) if !s.trim().is_empty() => {
            Uuid::parse_str(s).map(Some).map_err(serde::de::Error::custom)
        }
        _ => Ok(None),
    }
}

fn deserialize_can_have_children<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize as _;
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(match s.as_deref() {
        Some("Yes") => true,
        Some("No") => false,
        _ => {
            return Err(serde::de::Error::custom("Invalid value for CanHaveChildren"));
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gamut {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "@Points")]
    pub points: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gamuts {
    #[serde(default, rename = "Gamut")]
    pub gamuts: Vec<Gamut>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "GDTF")]
pub struct GdtfDescription {
    #[serde(rename = "@DataVersion")]
    pub data_version: DataVersion,
    #[serde(rename = "FixtureType")]
    pub fixture_types: Vec<FixtureType>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Geometries {
    #[serde(default, rename = "$value")]
    pub geometries: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeometryReference {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "Break")]
    pub breaks: Vec<Break>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum InterpolationTo {
    #[default]
    #[serde(rename = "Linear")]
    Linear,
    #[serde(rename = "Step")]
    Step,
    #[serde(rename = "Log")]
    Log,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Inventory {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "@Count")]
    pub count: Option<i32>,
    #[serde(default, rename = "Break")]
    pub breaks: Vec<Break>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LampType {
    #[serde(rename = "Discharge")]
    Discharge,
    #[serde(rename = "Tungsten")]
    Tungsten,
    #[serde(rename = "Halogen")]
    Halogen,
    #[serde(rename = "LED")]
    Led,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Laser {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(default = "Laser::default_color_type", rename = "@ColorType")]
    pub color_type: LaserColorType,
    #[serde(default, rename = "@Color")]
    pub color: Option<f32>,
    #[serde(default, rename = "@OutputStrength")]
    pub output_strength: Option<f32>,
    #[serde(default, rename = "@Emitter")]
    pub emitter: Option<Name>,
    #[serde(default, rename = "@BeamDiameter")]
    pub beam_diameter: Option<f32>,
    #[serde(default, rename = "@BeamDivergenceMin")]
    pub beam_divergence_min: Option<f32>,
    #[serde(default, rename = "@BeamDivergenceMax")]
    pub beam_divergence_max: Option<f32>,
    #[serde(default, rename = "@ScanAnglePan")]
    pub scan_angle_pan: Option<f32>,
    #[serde(default, rename = "@ScanAngleTilt")]
    pub scan_angle_tilt: Option<f32>,
    #[serde(default, rename = "@ScanSpeed")]
    pub scan_speed: Option<f32>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
    #[serde(default, rename = "Protocol")]
    pub protocols: Vec<Protocol>,
}

impl Laser {
    pub fn default_color_type() -> LaserColorType {
        LaserColorType::Rgb
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LaserColorType {
    #[serde(rename = "RGB")]
    Rgb,
    #[serde(rename = "SingleWaveLength")]
    SingleWaveLength,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LegHeight {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Weight {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalChannel {
    #[serde(rename = "@Attribute")]
    pub attribute: Name,
    #[serde(default, rename = "@Snap")]
    pub snap: Snap,
    #[serde(default, rename = "@Master")]
    pub master: Master,
    #[serde(default, rename = "@MibFade")]
    pub mib_fade: Option<f32>,
    #[serde(default, rename = "@DMXChangeTimeLimit")]
    pub dmx_change_time_limit: Option<f32>,
    #[serde(default, rename = "ChannelFunction")]
    pub channel_functions: Vec<ChannelFunction>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmx {
    #[serde(default, rename = "MacroDMXStep")]
    pub macro_dmx_steps: Vec<MacroDmxStep>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmxStep {
    #[serde(default, rename = "@Duration")]
    pub duration: Option<f32>,
    #[serde(default, rename = "MacroDMXValue")]
    pub macro_dmx_value: Vec<MacroDmxValue>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MacroDmxValue {
    #[serde(rename = "@Value")]
    pub value: DmxValue,
    #[serde(rename = "@DMXChannel")]
    pub dmx_channel: Node,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Map {
    #[serde(default, rename = "@Key")]
    pub key: Option<u32>,
    #[serde(default, rename = "@Value")]
    pub value: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Master {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Grand")]
    Grand,
    #[serde(rename = "Group")]
    Group,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeasurementPoint {
    #[serde(rename = "@WaveLength")]
    pub wave_length: f32,
    #[serde(rename = "@Energy")]
    pub energy: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Model {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Length")]
    pub length: f32,
    #[serde(default, rename = "@Width")]
    pub width: f32,
    #[serde(default, rename = "@Height")]
    pub height: f32,
    #[serde(default, rename = "@PrimitiveType")]
    pub primitive_type: PrimitiveType,
    #[serde(default, rename = "@File")]
    pub file: String,
    #[serde(default, rename = "@SVGOffsetX")]
    pub svg_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGOffsetY")]
    pub svg_offset_y: Option<f32>,
    #[serde(default, rename = "@SVGSideOffsetX")]
    pub svg_side_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGSideOffsetY")]
    pub svg_side_offset_y: Option<f32>,
    #[serde(default, rename = "@SVGFrontOffsetX")]
    pub svg_front_offset_x: Option<f32>,
    #[serde(default, rename = "@SVGFrontOffsetY")]
    pub svg_front_offset_y: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Models {
    #[serde(default, rename = "Model")]
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OperatingTemperature {
    #[serde(default, rename = "@Low")]
    pub low: Option<f32>,
    #[serde(default, rename = "@High")]
    pub high: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhysicalDescriptions {
    #[serde(default, rename = "Emitters")]
    pub emitters: Option<Emitters>,
    #[serde(default, rename = "Filters")]
    pub filters: Option<Filters>,
    #[serde(default, rename = "ColorSpace")]
    pub color_space: Option<ColorSpace>,
    #[serde(default, rename = "AdditionalColorSpaces")]
    pub additional_color_spaces: Option<AdditionalColorSpaces>,
    #[serde(default, rename = "Gamuts")]
    pub gamuts: Option<Gamuts>,
    #[serde(default, rename = "DMXProfiles")]
    pub dmx_profiles: Option<DmxProfiles>,
    #[serde(default, rename = "CRIs")]
    pub cr_is: Option<Cris>,
    #[serde(default, rename = "Connectors")]
    pub connectors: Option<Connectors>,
    #[serde(default, rename = "Properties")]
    pub properties: Option<Properties>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PhysicalUnitType {
    #[default]
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Percent")]
    Percent,
    #[serde(rename = "Length")]
    Length,
    #[serde(rename = "Mass")]
    Mass,
    #[serde(rename = "Time")]
    Time,
    #[serde(rename = "Temperature")]
    Temperature,
    #[serde(rename = "LuminousIntensity")]
    LuminousIntensity,
    #[serde(rename = "Angle")]
    Angle,
    #[serde(rename = "Force")]
    Force,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Current")]
    Current,
    #[serde(rename = "Voltage")]
    Voltage,
    #[serde(rename = "Power")]
    Power,
    #[serde(rename = "Energy")]
    Energy,
    #[serde(rename = "Area")]
    Area,
    #[serde(rename = "Volume")]
    Volume,
    #[serde(rename = "Speed")]
    Speed,
    #[serde(rename = "Acceleration")]
    Acceleration,
    #[serde(rename = "AngularSpeed")]
    AngularSpeed,
    #[serde(rename = "AngularAccc")]
    AngularAccc,
    #[serde(rename = "WaveLength")]
    WaveLength,
    #[serde(rename = "ColorComponent")]
    ColorComponent,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PinPatch {
    #[serde(default, rename = "@ToWiringObject")]
    pub to_wiring_object: Option<Node>,
    #[serde(default, rename = "@FromPin")]
    pub from_pin: Option<i32>,
    #[serde(default, rename = "@ToPin")]
    pub to_pin: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Point {
    #[serde(default, rename = "@DMXPercentage")]
    pub dmx_percentage: Option<f32>,
    #[serde(default, rename = "@CFC0")]
    pub cfc_0: Option<f32>,
    #[serde(default, rename = "@CFC1")]
    pub cfc_1: Option<f32>,
    #[serde(default, rename = "@CFC2")]
    pub cfc_2: Option<f32>,
    #[serde(default, rename = "@CFC3")]
    pub cfc_3: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PowerConsumption {
    #[serde(default, rename = "@Value")]
    pub value: Option<f32>,
    #[serde(default, rename = "@PowerFactor")]
    pub power_factor: Option<f32>,
    #[serde(rename = "@Connector")]
    pub connector: Name,
    #[serde(default, rename = "@VoltageLow")]
    pub voltage_low: Option<f32>,
    #[serde(default, rename = "@VoltageHigh")]
    pub voltage_high: Option<f32>,
    #[serde(default, rename = "@FrequencyLow")]
    pub frequency_low: Option<f32>,
    #[serde(default, rename = "@FrequencyHigh")]
    pub frequency_high: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PrimitiveType {
    #[default]
    #[serde(rename = "Undefined")]
    Undefined,
    #[serde(rename = "Cube")]
    Cube,
    #[serde(rename = "Cylinder")]
    Cylinder,
    #[serde(rename = "Sphere")]
    Sphere,
    #[serde(rename = "Base")]
    Base,
    #[serde(rename = "Yoke")]
    Yoke,
    #[serde(rename = "Head")]
    Head,
    #[serde(rename = "Scanner")]
    Scanner,
    #[serde(rename = "Conventional")]
    Conventional,
    #[serde(rename = "Pigtail")]
    Pigtail,
    #[serde(rename = "Base1_1")]
    Base11,
    #[serde(rename = "Scanner1_1")]
    Scanner11,
    #[serde(rename = "Conventional1_1")]
    Conventional11,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Properties {
    #[serde(default, rename = "$value")]
    pub content: Vec<PropertiesContent>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PropertiesContent {
    #[serde(rename = "OperatingTemperature")]
    OperatingTemperature(OperatingTemperature),
    #[serde(rename = "Weight")]
    Weight(Weight),
    #[serde(rename = "PowerConsumption")]
    PowerConsumption(PowerConsumption),
    #[serde(rename = "LegHeight")]
    LegHeight(LegHeight),
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocol {
    #[serde(default, rename = "@Name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Protocols {
    #[serde(default, rename = "FTRDM")]
    pub ftrdm: Option<FtRdm>,
    #[serde(default, rename = "Art-Net")]
    pub art_net: Option<ArtNet>,
    #[serde(default, rename = "sACN")]
    pub s_acn: Option<SAcn>,
    #[serde(default, rename = "PosiStageNet")]
    pub posi_stage_net: Option<PosiStageNet>,
    #[serde(default, rename = "OpenSoundControl")]
    pub open_sound_control: Option<OpenSoundControl>,
    #[serde(default, rename = "CITP")]
    pub citp: Option<Citp>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Relation {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(rename = "@Master")]
    pub master: Node,
    #[serde(rename = "@Follower")]
    pub follower: Node,
    #[serde(rename = "@Type")]
    pub type_: RelationType,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RelationType {
    #[serde(rename = "Multiply")]
    Multiply,
    #[serde(rename = "Override")]
    Override,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Relations {
    #[serde(default, rename = "Relation")]
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Revision {
    #[serde(default, rename = "@Text")]
    pub text: String,
    #[serde(default, rename = "@Date")]
    pub date: Option<String>,
    #[serde(default, rename = "@UserID")]
    pub user_id: i32,
    #[serde(default, rename = "@ModifiedBy")]
    pub modified_by: String,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Revisions {
    #[serde(default, rename = "Revision")]
    pub revisions: Vec<Revision>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Slot {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(default, rename = "@Color")]
    pub color: Option<Vector3>,
    #[serde(default, rename = "@Filter")]
    pub filter: Option<Name>,
    #[serde(default, rename = "@MediaFileName")]
    pub media_file_name: String,
    #[serde(default, rename = "Facet")]
    pub facets: Vec<Facet>,
    #[serde(default, rename = "AnimationSystem")]
    pub animation_systems: Vec<AnimationSystem>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Snap {
    #[serde(rename = "Yes")]
    Yes,
    #[default]
    #[serde(rename = "No")]
    No,
    #[serde(rename = "On")]
    On,
    #[serde(rename = "Off")]
    Off,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SoftwareVersionId {
    #[serde(default, rename = "@Value")]
    pub value: Option<String>,
    #[serde(default, rename = "DMXPersonality")]
    pub dmx_personality: Vec<DmxPersonality>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Structure {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "@LinkedGeometry")]
    pub linked_geometry: Option<Name>,
    #[serde(default, rename = "@StructureType")]
    pub structure_type: Option<StructureType>,
    #[serde(default, rename = "@CrossSectionType")]
    pub cross_section_type: Option<CrossSectionType>,
    #[serde(default, rename = "@CrossSectionHeight")]
    pub cross_section_height: Option<f32>,
    #[serde(default, rename = "@CrossSectionWallThickness")]
    pub cross_section_wall_thickness: Option<f32>,
    #[serde(default, rename = "@TrussCrossSection")]
    pub truss_cross_section: Option<String>,
    #[serde(default, rename = "Break")]
    pub breaks: Vec<Break>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum StructureType {
    #[serde(rename = "CenterLineBased")]
    CenterLineBased,
    #[serde(rename = "Detail")]
    Detail,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubChannelSet {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<String>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<String>,
    #[serde(default, rename = "@SubPhysicalUnit")]
    pub sub_physical_unit: Option<Node>,
    #[serde(default, rename = "@DMXProfile")]
    pub dmx_profile: Option<Node>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SubPhysicalType {
    #[serde(rename = "PlacementOffset")]
    PlacementOffset,
    #[serde(rename = "Amplitude")]
    Amplitude,
    #[serde(rename = "AmplitudeMin")]
    AmplitudeMin,
    #[serde(rename = "Duration")]
    Duration,
    #[serde(rename = "DutyCycle")]
    DutyCycle,
    #[serde(rename = "TimeOffset")]
    TimeOffset,
    #[serde(rename = "MinimumOpening")]
    MinimumOpening,
    #[serde(rename = "Value")]
    Value,
    #[serde(rename = "RatioHorizontal")]
    RatioHorizontal,
    #[serde(rename = "RatioVertical")]
    RatioVertical,
    #[serde(rename = "AmplitudeMax")]
    AmplitudeMax,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubPhysicalUnit {
    #[serde(default, rename = "@Type")]
    pub type_: Option<SubPhysicalType>,
    #[serde(default, rename = "@PhysicalUnit")]
    pub physical_unit: Option<PhysicalUnitType>,
    #[serde(default, rename = "@PhysicalFrom")]
    pub physical_from: Option<f32>,
    #[serde(default, rename = "@PhysicalTo")]
    pub physical_to: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Support {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<Name>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(rename = "@Geometry")]
    pub geometry: Name,
    #[serde(default, rename = "@SupportType")]
    pub support_type: Option<SupportType>,
    #[serde(default, rename = "@RopeCrossSection")]
    pub rope_cross_section: Option<String>,
    #[serde(
        default,
        rename = "@RopeOffset",
        deserialize_with = "deserialize_option_from_string_none",
        serialize_with = "serialize_option_as_string_none"
    )]
    pub rope_offset: Option<Vector3>,
    #[serde(default, rename = "@CapacityX")]
    pub capacity_x: Option<f32>,
    #[serde(default, rename = "@CapacityY")]
    pub capacity_y: Option<f32>,
    #[serde(default, rename = "@CapacityZ")]
    pub capacity_z: Option<f32>,
    #[serde(default, rename = "@CapacityXX")]
    pub capacity_xx: Option<f32>,
    #[serde(default, rename = "@CapacityYY")]
    pub capacity_yy: Option<f32>,
    #[serde(default, rename = "@CapacityZZ")]
    pub capacity_zz: Option<f32>,
    #[serde(default, rename = "@ResistanceX")]
    pub resistance_x: Option<f32>,
    #[serde(default, rename = "@ResistanceY")]
    pub resistance_y: Option<f32>,
    #[serde(default, rename = "@ResistanceZ")]
    pub resistance_z: Option<f32>,
    #[serde(default, rename = "@ResistanceXX")]
    pub resistance_xx: Option<f32>,
    #[serde(default, rename = "@ResistanceYY")]
    pub resistance_yy: Option<f32>,
    #[serde(default, rename = "@ResistanceZZ")]
    pub resistance_zz: Option<f32>,
    #[serde(default, rename = "Break")]
    pub breaks: Vec<Break>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SupportType {
    #[serde(rename = "Rope")]
    Rope,
    #[serde(rename = "GroundSupport")]
    GroundSupport,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wheel {
    #[serde(default, rename = "@Name")]
    pub name: Option<Name>,
    #[serde(default, rename = "Slot")]
    pub slots: Vec<Slot>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wheels {
    #[serde(default, rename = "Wheel")]
    pub wheels: Vec<Wheel>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringComponentType {
    #[serde(rename = "Input")]
    Input,
    #[serde(rename = "Output")]
    Output,
    #[serde(rename = "PowerSource")]
    PowerSource,
    #[serde(rename = "Consumer")]
    Consumer,
    #[serde(rename = "Fuse")]
    Fuse,
    #[serde(rename = "NetworkProvider")]
    NetworkProvider,
    #[serde(rename = "NetworkInput")]
    NetworkInput,
    #[serde(rename = "NetworkOutput")]
    NetworkOutput,
    #[serde(rename = "NetworkInOut")]
    NetworkInOut,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringFuseRating {
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "K")]
    K,
    #[serde(rename = "Z")]
    Z,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WiringObject {
    #[serde(rename = "@Name")]
    pub name: Name,
    #[serde(default, rename = "@Model")]
    pub model: Option<String>,
    #[serde(default, rename = "@Position")]
    pub position: Matrix4x4,
    #[serde(default, rename = "@ConnectorType")]
    pub connector_type: Option<String>,
    #[serde(default, rename = "@ComponentType")]
    pub component_type: Option<WiringComponentType>,
    #[serde(default, rename = "@SignalType")]
    pub signal_type: Option<String>,
    #[serde(default, rename = "@PinCount")]
    pub pin_count: Option<i32>,
    #[serde(default, rename = "@ElectricalPayLoad")]
    pub electrical_pay_load: Option<f32>,
    #[serde(default, rename = "@VoltageRangeMax")]
    pub voltage_range_max: Option<f32>,
    #[serde(default, rename = "@VoltageRangeMin")]
    pub voltage_range_min: Option<f32>,
    #[serde(default, rename = "@FrequencyRangeMax")]
    pub frequency_range_max: Option<f32>,
    #[serde(default, rename = "@FrequencyRangeMin")]
    pub frequency_range_min: Option<f32>,
    #[serde(default, rename = "@MaxPayLoad")]
    pub max_pay_load: Option<f32>,
    #[serde(default, rename = "@Voltage")]
    pub voltage: Option<f32>,
    #[serde(default, rename = "@SignalLayer")]
    pub signal_layer: Option<i32>,
    #[serde(default, rename = "@CosPhi")]
    pub cos_phi: Option<f32>,
    #[serde(default, rename = "@FuseCurrent")]
    pub fuse_current: Option<f32>,
    #[serde(default, rename = "@FuseRating")]
    pub fuse_rating: Option<WiringFuseRating>,
    #[serde(default, rename = "@Orientation")]
    pub orientation: Option<WiringOrientation>,
    #[serde(default, rename = "@WireGroup")]
    pub wire_group: Option<String>,
    #[serde(default, rename = "$value")]
    pub children: Vec<AnyGeometry>,
    #[serde(default, rename = "PinPatch")]
    pub pin_patches: Vec<PinPatch>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WiringOrientation {
    #[serde(rename = "Left")]
    Left,
    #[serde(rename = "Right")]
    Right,
    #[serde(rename = "Top")]
    Top,
    #[serde(rename = "Bottom")]
    Bottom,
}
