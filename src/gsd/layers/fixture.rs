use std::net::{Ipv4Addr, Ipv6Addr};

use uuid::Uuid;

use crate::{CieColor, FileName, Gobo, Matrix4x3, deserialize_matrix_option};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Fixture")]
pub struct Fixture {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,
    #[serde(rename = "@multipatch", default)]
    pub(crate) multipatch: Option<String>,

    #[serde(rename = "Matrix", default, deserialize_with = "deserialize_matrix_option")]
    pub(crate) matrix: Option<Matrix4x3>,
    #[serde(rename = "Classing", default)]
    pub(crate) classing: Option<Uuid>,
    #[serde(rename = "GDTFSpec", default)]
    pub(crate) gdtf_spec: Option<FileName>,
    #[serde(rename = "GDTFMode", default)]
    pub(crate) gdtf_mode: Option<String>,
    #[serde(rename = "Focus", default)]
    pub(crate) focus: Option<Uuid>,
    #[serde(rename = "CastShadow", default)]
    pub(crate) cast_shadow: Option<bool>,
    #[serde(rename = "DMXInvertPan", default)]
    pub(crate) dmx_invert_pan: Option<bool>,
    #[serde(rename = "DMXInvertTilt", default)]
    pub(crate) dmx_invert_tilt: Option<bool>,
    #[serde(rename = "Position", default)]
    pub(crate) position: Option<Uuid>,
    #[serde(rename = "Function", default)]
    pub(crate) function: Option<String>,
    #[serde(rename = "FixtureID", default)]
    pub(crate) fixture_id: Option<String>,
    #[serde(rename = "FixtureIDNumeric", default)]
    pub(crate) fixture_id_numeric: Option<i32>,
    #[serde(rename = "UnitNumber", default)]
    pub(crate) unit_number: Option<i32>,
    #[serde(rename = "FixtureTypeId", default)]
    pub(crate) fixture_type_id: Option<i32>,
    #[serde(rename = "CustomId", default)]
    pub(crate) custom_id: Option<i32>,
    #[serde(rename = "ChildPosition", default)]
    pub(crate) child_position: Option<String>,
    #[serde(rename = "Addresses", default)]
    pub(crate) addresses: Option<Addresses>,
    #[serde(rename = "Protocols", default)]
    pub(crate) protocols: Option<Protocols>,
    #[serde(rename = "Alignments", default)]
    pub(crate) alignments: Option<Alignments>,
    #[serde(rename = "CustomCommands", default)]
    pub(crate) custom_commands: Option<CustomCommands>,
    #[serde(rename = "Overwrites", default)]
    pub(crate) overwrites: Option<Overwrites>,
    #[serde(rename = "Connections", default)]
    pub(crate) connections: Option<Connections>,
    #[serde(rename = "Color", default)]
    pub(crate) color: Option<CieColor>,
    #[serde(rename = "CustomIdType", default)]
    pub(crate) custom_id_type: Option<i32>,
    #[serde(rename = "Mappings", default)]
    pub(crate) mappings: Option<Mappings>,
    #[serde(rename = "Gobo", default)]
    pub(crate) gobo: Option<Gobo>,
    #[serde(rename = "ChildList", default)]
    pub(crate) child_list: FixtureChildList,
}

impl Fixture {
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn multipatch(&self) -> Option<&str> {
        self.multipatch.as_deref()
    }

    pub fn matrix(&self) -> Option<&Matrix4x3> {
        self.matrix.as_ref()
    }

    pub fn classing(&self) -> Option<Uuid> {
        self.classing
    }

    pub fn gdtf_spec(&self) -> Option<&FileName> {
        self.gdtf_spec.as_ref()
    }

    pub fn gdtf_mode(&self) -> Option<&str> {
        self.gdtf_mode.as_deref()
    }

    pub fn focus(&self) -> Option<&Uuid> {
        self.focus.as_ref()
    }

    pub fn cast_shadow(&self) -> Option<bool> {
        self.cast_shadow
    }

    pub fn dmx_invert_pan(&self) -> Option<bool> {
        self.dmx_invert_pan
    }

    pub fn dmx_invert_tilt(&self) -> Option<bool> {
        self.dmx_invert_tilt
    }

    pub fn position(&self) -> Option<&Uuid> {
        self.position.as_ref()
    }

    pub fn function(&self) -> Option<&str> {
        self.function.as_deref()
    }

    pub fn fixture_id(&self) -> Option<&str> {
        self.fixture_id.as_deref()
    }

    pub fn fixture_id_numeric(&self) -> Option<i32> {
        self.fixture_id_numeric
    }

    /// Returns the unit number, falling back to `fixture_id_numeric `
    /// or `fixture_id` if `unit_number` is not set.
    pub fn unit_number(&self) -> i32 {
        if let Some(unit_number) = self.unit_number {
            unit_number
        } else if let Some(numeric_fid) = self.fixture_id_numeric {
            numeric_fid
        } else if let Some(ref fixture_id) = self.fixture_id {
            fixture_id.parse::<i32>().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn fixture_type_id(&self) -> Option<i32> {
        self.fixture_type_id
    }

    pub fn child_position(&self) -> Option<&str> {
        self.child_position.as_deref()
    }

    pub fn addresses(&self) -> Option<&Addresses> {
        self.addresses.as_ref()
    }

    pub fn protocols(&self) -> Option<&Protocols> {
        self.protocols.as_ref()
    }

    pub fn alignments(&self) -> Option<&Alignments> {
        self.alignments.as_ref()
    }

    pub fn custom_commands(&self) -> Option<&CustomCommands> {
        self.custom_commands.as_ref()
    }

    pub fn overwrites(&self) -> Option<&Overwrites> {
        self.overwrites.as_ref()
    }

    pub fn connections(&self) -> Option<&Connections> {
        self.connections.as_ref()
    }

    pub fn custom_id_type(&self) -> Option<i32> {
        self.custom_id_type
    }

    pub fn custom_id(&self) -> Option<i32> {
        self.custom_id
    }

    pub fn mappings(&self) -> Option<&Mappings> {
        self.mappings.as_ref()
    }

    pub fn gobo(&self) -> Option<&Gobo> {
        self.gobo.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Addresses")]
pub struct Addresses {
    #[serde(rename = "Address", default)]
    pub(crate) addresses: Vec<Address>,
    #[serde(rename = "Network", default)]
    pub(crate) networkes: Vec<Network>,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Address")]
pub struct Address {
    #[serde(rename = "@break", default)]
    pub(crate) r#break: i32,

    #[serde(rename = "$value", default)]
    pub(crate) value: AddressValue,
}

impl Address {
    pub fn r#break(&self) -> i32 {
        self.r#break
    }

    pub fn value(&self) -> &AddressValue {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize)]
pub enum AddressValue {
    Integer(i32),
    UniverseAddress { universe: i32, address: i32 },
}

impl<'de> serde::Deserialize<'de> for AddressValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AddressValueVisitor;

        impl<'de> serde::de::Visitor<'de> for AddressValueVisitor {
            type Value = AddressValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer or '<universe>.<address>' string for AddressValue")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let parts: Vec<&str> = v.split('.').collect();
                if parts.len() == 2 {
                    let universe = parts[0]
                        .parse::<i32>()
                        .map_err(|_| E::custom(format!("Invalid universe number in '{}'", v)))?;
                    let address = parts[1]
                        .parse::<i32>()
                        .map_err(|_| E::custom(format!("Invalid address number in '{}'", v)))?;
                    return Ok(AddressValue::UniverseAddress { universe, address });
                }

                if let Ok(val) = v.parse::<i32>() {
                    return Ok(AddressValue::Integer(val));
                }
                Err(E::custom(format!(
                    "Invalid AddressValue string '{}', expected integer or '<universe>.<address>'",
                    v
                )))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(AddressValue::Integer(v as i32))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(AddressValue::Integer(v as i32))
            }
        }

        deserializer.deserialize_any(AddressValueVisitor)
    }
}

impl Default for AddressValue {
    fn default() -> Self {
        AddressValue::Integer(1)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Network")]
pub struct Network {
    #[serde(rename = "@geometry")]
    pub(crate) geometry: String,
    #[serde(rename = "@ipv4", default)]
    pub(crate) ipv4: Option<Ipv4Addr>,
    #[serde(rename = "@subnetmask", default)]
    pub(crate) subnetmask: Option<Ipv4Addr>,
    #[serde(rename = "@ipv6", default)]
    pub(crate) ipv6: Option<Ipv6Addr>,
    #[serde(rename = "@dhcp", default)]
    pub(crate) dhcp: bool,
    #[serde(rename = "@hostname", default)]
    pub(crate) hostname: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Protocols")]
pub struct Protocols {
    #[serde(rename = "Protocol", default)]
    pub(crate) protocols: Vec<Protocol>,
}

impl std::ops::Deref for Protocols {
    type Target = Vec<Protocol>;

    fn deref(&self) -> &Self::Target {
        &self.protocols
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Protocol")]
pub struct Protocol {
    #[serde(rename = "@geometry", default = "default_protocol_geometry_name")]
    pub(crate) geometry: String,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,
    #[serde(rename = "@type", default)]
    pub(crate) protocol_type: String,
    #[serde(rename = "@version", default)]
    pub(crate) version: String,
    #[serde(rename = "@transmission")]
    pub(crate) transmission: Option<TransmissionType>,
}

impl Protocol {
    pub fn geometry(&self) -> &str {
        &self.geometry
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn protocol_type(&self) -> &str {
        &self.protocol_type
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn transmission(&self) -> Option<TransmissionType> {
        self.transmission
    }
}

fn default_protocol_geometry_name() -> String {
    "NetworkInOut_1".to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "TransmissionType")]
pub enum TransmissionType {
    #[serde(rename = "Unicast")]
    Unicast,
    #[serde(rename = "Multicast")]
    Multicast,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Anycast")]
    Anycast,
    #[default]
    #[serde(rename = "undefined")]
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Alignments")]
pub struct Alignments {
    #[serde(rename = "Alignment", default)]
    pub(crate) alignments: Vec<Alignment>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Alignment")]
pub struct Alignment {
    #[serde(rename = "@geometry")]
    pub(crate) geometry: String,
    #[serde(rename = "@up", default = "default_up")]
    pub(crate) up: String,
    #[serde(rename = "@direction", default = "default_direction")]
    pub(crate) direction: String,
}

fn default_up() -> String {
    "0,0,1".to_string()
}

fn default_direction() -> String {
    "0,0,-1".to_string()
}
#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "CustomCommands")]
pub struct CustomCommands {
    #[serde(rename = "CustomCommand", default)]
    pub(crate) custom_commands: Vec<CustomCommand>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "CustomCommand")]
pub struct CustomCommand {
    #[serde(rename = "$text", default)]
    pub command: String,
}
#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Overwrites")]
pub struct Overwrites {
    #[serde(rename = "Overwrite", default)]
    pub(crate) overwrites: Vec<Overwrite>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Overwrite")]
pub struct Overwrite {
    #[serde(rename = "@universal")]
    pub(crate) universal: String,
    #[serde(rename = "@target", default)]
    pub(crate) target: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Connections")]
pub struct Connections {
    #[serde(rename = "Connection", default)]
    pub(crate) connections: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Connection")]
pub struct Connection {
    #[serde(rename = "@own")]
    pub(crate) own: String,
    #[serde(rename = "@other")]
    pub(crate) other: String,
    #[serde(rename = "@toObject")]
    pub(crate) to_object: Uuid,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Mappings")]
pub struct Mappings {
    #[serde(rename = "Mapping", default)]
    pub(crate) mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Mapping")]
pub struct Mapping {
    #[serde(rename = "@linkedDef")]
    pub(crate) linked_def: Uuid,
    #[serde(rename = "ux")]
    pub(crate) ux: Option<i32>,
    #[serde(rename = "uy")]
    pub(crate) uy: Option<i32>,
    #[serde(rename = "ox")]
    pub(crate) ox: Option<i32>,
    #[serde(rename = "oy")]
    pub(crate) oy: Option<i32>,
    #[serde(rename = "rz")]
    pub(crate) rz: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "FixtureChildList")]
pub struct FixtureChildList {
    #[serde(rename = "Fixture", default)]
    pub(crate) fixtures: Vec<Fixture>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use uuid::Uuid;

    fn uuid(s: &str) -> Uuid {
        Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn test_fixture_minimal() {
        let xml = r#"<Fixture uuid="11111111-2222-3333-4444-555555555555"/>"#;
        let fixture: Fixture = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(fixture.uuid(), &uuid("11111111-2222-3333-4444-555555555555"));
        assert_eq!(fixture.name(), "");
        assert!(fixture.multipatch().is_none());
        assert!(fixture.matrix().is_none());
        assert!(fixture.classing().is_none());
        assert!(fixture.gdtf_spec().is_none());
        assert!(fixture.gdtf_mode().is_none());
        assert!(fixture.focus().is_none());
        assert!(fixture.cast_shadow().is_none());
        assert!(fixture.dmx_invert_pan().is_none());
        assert!(fixture.dmx_invert_tilt().is_none());
        assert!(fixture.position().is_none());
        assert!(fixture.function().is_none());
        assert!(fixture.fixture_id().is_none());
        assert!(fixture.fixture_id_numeric().is_none());
        assert_eq!(fixture.unit_number(), 0);
        assert!(fixture.fixture_type_id().is_none());
        assert!(fixture.child_position().is_none());
        assert!(fixture.addresses().is_none());
        assert!(fixture.protocols().is_none());
        assert!(fixture.alignments().is_none());
        assert!(fixture.custom_commands().is_none());
        assert!(fixture.overwrites().is_none());
        assert!(fixture.connections().is_none());
        assert!(fixture.color.is_none());
        assert!(fixture.custom_id_type().is_none());
        assert!(fixture.custom_id().is_none());
        assert!(fixture.mappings().is_none());
        assert!(fixture.gobo().is_none());
        assert!(fixture.child_list.fixtures.is_empty());
    }

    #[test]
    fn test_fixture_full() {
        let xml = r#"
          <Fixture uuid="11111111-2222-3333-4444-555555555555" name="TestFixture" multipatch="22222222-3333-4444-5555-666666666666">
              <Matrix>{1,0,0}{0,1,0}{0,0,1}{100,200,300}</Matrix>
              <Classing>aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee</Classing>
              <GDTFSpec>FixtureSpec.gdtf</GDTFSpec>
              <GDTFMode>Mode1</GDTFMode>
              <Focus>bbbbbbbb-cccc-dddd-eeee-ffffffffffff</Focus>
              <CastShadow>true</CastShadow>
              <DMXInvertPan>true</DMXInvertPan>
              <DMXInvertTilt>false</DMXInvertTilt>
              <Position>cccccccc-dddd-eeee-ffff-111111111111</Position>
              <Function>Spot</Function>
              <FixtureID>F1</FixtureID>
              <FixtureIDNumeric>101</FixtureIDNumeric>
              <UnitNumber>5</UnitNumber>
              <FixtureTypeId>7</FixtureTypeId>
              <CustomId>42</CustomId>
              <ChildPosition>Base.Yoke.Head</ChildPosition>
              <Addresses>
                  <Address break="1">2.45</Address>
                  <Network geometry="ethernet_1" ipv4="192.168.1.10" subnetmask="255.255.0.0" />
              </Addresses>
              <Protocols>
                  <Protocol geometry="Net1" name="ArtNet" type="DMX" version="4" transmission="Unicast"/>
              </Protocols>
              <Alignments>
                  <Alignment geometry="Beam" up="0,0,1" direction="0,0,-1"/>
              </Alignments>
              <CustomCommands>
                  <CustomCommand command="Body_Pan,f 50"/>
              </CustomCommands>
              <Overwrites>
                  <Overwrite universal="Universal Wheel 1.Universal Wheel Slot 1" target="Wheel 1.Wheel Slot"/>
              </Overwrites>
              <Connections>
                  <Connection own="Input" toObject="11111111-2222-3333-4444-555555555555" other="Output1"/>
              </Connections>
              <Color>0.314303,0.328065,87.699166</Color>
              <CustomIdType>1</CustomIdType>
              <Mappings>
                  <Mapping linkedDef="bbbbbbbb-cccc-dddd-eeee-ffffffffffff">
                      <ux>10</ux>
                      <uy>10</uy>
                      <ox>5</ox>
                      <oy>5</oy>
                      <rz>45</rz>
                  </Mapping>
              </Mappings>
              <Gobo rotation="32.5">image_file_forgobo</Gobo>
              <ChildList>
                  <Fixture uuid="22222222-3333-4444-5555-666666666666" name="ChildFixture"/>
              </ChildList>
          </Fixture>
          "#;
        let fixture: Fixture = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(fixture.uuid(), &uuid("11111111-2222-3333-4444-555555555555"));
        assert_eq!(fixture.name(), "TestFixture");
        assert_eq!(fixture.multipatch(), Some("22222222-3333-4444-5555-666666666666"));
        assert!(fixture.matrix().is_some());
        assert_eq!(fixture.classing(), Some(uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")));
        assert_eq!(fixture.gdtf_spec().unwrap().as_str(), "FixtureSpec.gdtf");
        assert_eq!(fixture.gdtf_mode(), Some("Mode1"));
        assert_eq!(fixture.focus(), Some(&uuid("bbbbbbbb-cccc-dddd-eeee-ffffffffffff")));
        assert_eq!(fixture.cast_shadow(), Some(true));
        assert_eq!(fixture.dmx_invert_pan(), Some(true));
        assert_eq!(fixture.dmx_invert_tilt(), Some(false));
        assert_eq!(fixture.position(), Some(&uuid("cccccccc-dddd-eeee-ffff-111111111111")));
        assert_eq!(fixture.function(), Some("Spot"));
        assert_eq!(fixture.fixture_id(), Some("F1"));
        assert_eq!(fixture.fixture_id_numeric(), Some(101));
        assert_eq!(fixture.unit_number(), 5);
        assert_eq!(fixture.fixture_type_id(), Some(7));
        assert_eq!(fixture.custom_id(), Some(42));
        assert_eq!(fixture.child_position(), Some("Base.Yoke.Head"));
        assert!(fixture.addresses().is_some());
        assert!(fixture.protocols().is_some());
        assert!(fixture.alignments().is_some());
        assert!(fixture.custom_commands().is_some());
        assert!(fixture.overwrites().is_some());
        assert!(fixture.connections().is_some());
        assert!(fixture.color.is_some());
        assert_eq!(fixture.custom_id_type(), Some(1));
        assert!(fixture.mappings().is_some());
        assert!(fixture.gobo().is_some());
        assert_eq!(fixture.child_list.fixtures.len(), 1);
        assert_eq!(
            fixture.child_list.fixtures[0].uuid(),
            &uuid("22222222-3333-4444-5555-666666666666")
        );
    }
    #[test]
    fn test_fixture_nested_childlist() {
        let xml = r#"
          <Fixture uuid="11111111-2222-3333-4444-555555555555">
              <ChildList>
                  <Fixture uuid="22222222-3333-4444-5555-666666666666">
                      <ChildList>
                          <Fixture uuid="33333333-4444-5555-6666-777777777777"/>
                      </ChildList>
                  </Fixture>
              </ChildList>
          </Fixture>
          "#;
        let fixture: Fixture = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(fixture.child_list.fixtures.len(), 1);
        let child = &fixture.child_list.fixtures[0];
        assert_eq!(child.child_list.fixtures.len(), 1);
        let grandchild = &child.child_list.fixtures[0];
        assert_eq!(grandchild.uuid(), &uuid("33333333-4444-5555-6666-777777777777"));
    }

    #[test]
    fn test_custom_command_deserialize() {
        let xml = r#"<CustomCommand>Body_Pan,f 50</CustomCommand>"#;
        let cmd: CustomCommand = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cmd.command, "Body_Pan,f 50");
        let xml_out = quick_xml::se::to_string(&cmd).unwrap();
        assert_eq!(xml_out, r#"<CustomCommand>Body_Pan,f 50</CustomCommand>"#);
    }

    #[test]
    fn test_custom_commands_deserialize() {
        let xml = r#"
            <CustomCommands>
                <CustomCommand>Body_Pan,f 50</CustomCommand>
                <CustomCommand>Yoke_Tilt,f 50</CustomCommand>
            </CustomCommands>
        "#;
        let cmds: CustomCommands = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cmds.custom_commands.len(), 2);
        assert_eq!(cmds.custom_commands[0].command, "Body_Pan,f 50");
        assert_eq!(cmds.custom_commands[1].command, "Yoke_Tilt,f 50");
        let xml_out = quick_xml::se::to_string(&cmds).unwrap();
        let expected = r#"<CustomCommands><CustomCommand>Body_Pan,f 50</CustomCommand><CustomCommand>Yoke_Tilt,f 50</CustomCommand></CustomCommands>"#;
        assert_eq!(xml_out, expected);
    }

    #[test]
    fn test_overwrite_deserialize() {
        let xml = r#"<Overwrite universal="Universal" target="Target"/>"#;
        let ow: Overwrite = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(ow.universal, "Universal");
        assert_eq!(ow.target, "Target");
        let xml_out = quick_xml::se::to_string(&ow).unwrap();
        assert_eq!(xml_out, r#"<Overwrite universal="Universal" target="Target"/>"#);
    }

    #[test]
    fn test_overwrites_deserialize() {
        let xml = r#"
            <Overwrites>
                <Overwrite universal="U1" target="T1"/>
                <Overwrite universal="U2" target="T2"/>
            </Overwrites>
        "#;
        let ows: Overwrites = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(ows.overwrites.len(), 2);
        assert_eq!(ows.overwrites[0].universal, "U1");
        assert_eq!(ows.overwrites[1].target, "T2");
        let xml_out = quick_xml::se::to_string(&ows).unwrap();
        let expected = r#"<Overwrites><Overwrite universal="U1" target="T1"/><Overwrite universal="U2" target="T2"/></Overwrites>"#;
        assert_eq!(xml_out, expected);
    }

    #[test]
    fn test_alignment_defaults() {
        let xml = r#"<Alignment geometry="Beam"/>"#;
        let align: Alignment = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(align.geometry, "Beam");
        assert_eq!(align.up, "0,0,1");
        assert_eq!(align.direction, "0,0,-1");
        let xml_out = quick_xml::se::to_string(&align).unwrap();
        let expected = r#"<Alignment geometry="Beam" up="0,0,1" direction="0,0,-1"/>"#;
        assert_eq!(xml_out, expected);
    }

    #[test]
    fn test_alignments_deserialize() {
        let xml = r#"
            <Alignments>
                <Alignment geometry="Beam" up="1,0,0" direction="0,1,0"/>
            </Alignments>
        "#;
        let aligns: Alignments = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(aligns.alignments.len(), 1);
        assert_eq!(aligns.alignments[0].geometry, "Beam");
        assert_eq!(aligns.alignments[0].up, "1,0,0");
        assert_eq!(aligns.alignments[0].direction, "0,1,0");
        let xml_out = quick_xml::se::to_string(&aligns).unwrap();
        let expected =
            r#"<Alignments><Alignment geometry="Beam" up="1,0,0" direction="0,1,0"/></Alignments>"#;
        assert_eq!(xml_out, expected);
    }

    #[test]
    fn test_protocol_deserialize() {
        let xml = r#"<Protocol geometry="Net1" name="ArtNet" type="DMX" version="4" transmission="Unicast"/>"#;
        let protocol: Protocol = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(protocol.geometry, "Net1");
        assert_eq!(protocol.name, "ArtNet");
        assert_eq!(protocol.protocol_type, "DMX");
        assert_eq!(protocol.version, "4");
        assert_eq!(protocol.transmission, Some(TransmissionType::Unicast));
        let xml_out = quick_xml::se::to_string(&protocol).unwrap();
        assert_eq!(
            xml_out,
            r#"<Protocol geometry="Net1" name="ArtNet" type="DMX" version="4" transmission="Unicast"/>"#
        );
    }

    #[test]
    fn test_protocols_deserialize() {
        let xml = r#"
            <Protocols>
                <Protocol geometry="Net1" name="ArtNet" type="DMX" version="4" transmission="Unicast"/>
                <Protocol geometry="Net2" name="sACN" type="DMX" version="2" transmission="Multicast"/>
            </Protocols>
        "#;
        let protocols: Protocols = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(protocols.protocols.len(), 2);
        assert_eq!(protocols.protocols[1].name, "sACN");
    }

    #[test]
    fn test_addressvalue_integer_and_universe() {
        let xml = r#"<Address break="1">123</Address>"#;
        let addr: Address = quick_xml::de::from_str(xml).unwrap();
        match addr.value {
            AddressValue::Integer(val) => assert_eq!(val, 123),
            _ => panic!("Expected Integer"),
        }
        let xml2 = r#"<Address break="2">2.45</Address>"#;
        let addr2: Address = quick_xml::de::from_str(xml2).unwrap();
        match addr2.value {
            AddressValue::UniverseAddress { universe, address } => {
                assert_eq!(universe, 2);
                assert_eq!(address, 45);
            }
            _ => panic!("Expected UniverseAddress"),
        }
    }

    #[test]
    fn test_network_deserialize() {
        let xml = r#"<Network geometry="eth1" ipv4="192.168.1.10" subnetmask="255.255.255.0" ipv6="::1" dhcp="true" hostname="host1"/>"#;
        let net: Network = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(net.geometry, "eth1");
        assert_eq!(net.ipv4.unwrap().to_string(), "192.168.1.10");
        assert_eq!(net.subnetmask.unwrap().to_string(), "255.255.255.0");
        assert_eq!(net.ipv6.unwrap().to_string(), "::1");
        assert!(net.dhcp);
        assert_eq!(net.hostname.as_deref(), Some("host1"));
    }

    #[test]
    fn test_mappings_and_mapping() {
        let xml = r#"
            <Mappings>
                <Mapping linkedDef="bbbbbbbb-cccc-dddd-eeee-ffffffffffff">
                    <ux>10</ux>
                    <uy>20</uy>
                    <ox>30</ox>
                    <oy>40</oy>
                    <rz>45</rz>
                </Mapping>
            </Mappings>
        "#;
        let mappings: Mappings = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(mappings.mappings.len(), 1);
        let mapping = &mappings.mappings[0];
        assert_eq!(mapping.ux, Some(10));
        assert_eq!(mapping.uy, Some(20));
        assert_eq!(mapping.ox, Some(30));
        assert_eq!(mapping.oy, Some(40));
        assert_eq!(mapping.rz, Some(45.0));
    }

    #[test]
    fn test_connections_and_connection() {
        let xml = r#"
            <Connections>
                <Connection own="Input" toObject="11111111-2222-3333-4444-555555555555" other="Output1"/>
            </Connections>
        "#;
        let conns: Connections = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(conns.connections.len(), 1);
        let conn = &conns.connections[0];
        assert_eq!(conn.own, "Input");
        assert_eq!(conn.other, "Output1");
        assert_eq!(conn.to_object, uuid("11111111-2222-3333-4444-555555555555"));
    }

    #[test]
    fn test_fixture_child_list() {
        let xml = r#"
            <FixtureChildList>
                <Fixture uuid="11111111-2222-3333-4444-555555555555"/>
                <Fixture uuid="22222222-3333-4444-5555-666666666666"/>
            </FixtureChildList>
        "#;
        let cl: FixtureChildList = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cl.fixtures.len(), 2);
        assert_eq!(cl.fixtures[1].uuid(), &uuid("22222222-3333-4444-5555-666666666666"));
    }
}
