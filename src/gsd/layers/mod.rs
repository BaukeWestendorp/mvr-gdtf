use std::ops;

use uuid::Uuid;

use crate::{Matrix4x3, deserialize_matrix_option};

mod fixture;

pub use fixture::*;

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Layers")]
pub struct Layers {
    #[serde(rename = "Layer", default)]
    pub(crate) layers: Vec<Layer>,
}

impl ops::Deref for Layers {
    type Target = [Layer];

    fn deref(&self) -> &Self::Target {
        &self.layers
    }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Layer")]
pub struct Layer {
    #[serde(rename = "@uuid")]
    pub(crate) uuid: Uuid,
    #[serde(rename = "@name", default)]
    pub(crate) name: String,

    #[serde(rename = "Matrix", default, deserialize_with = "deserialize_matrix_option")]
    pub(crate) matrix: Option<Matrix4x3>,

    #[serde(rename = "ChildList", default)]
    pub(crate) child_list: LayerChildList,
}

impl ops::Deref for Layer {
    type Target = LayerChildList;

    fn deref(&self) -> &Self::Target {
        &self.child_list
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "LayerChildList")]
pub struct LayerChildList {
    // FIXME: #[serde(rename = "SceneObject", default)]
    //        scene_objects: Vec<SceneObject>,
    // FIXME: #[serde(rename = "GroupObject", default)]
    //        group_objects: Vec<GroupObject>,
    // FIXME: #[serde(rename = "FocusPoint", default)]
    //        focus_points: Vec<FocusPoint>,
    #[serde(rename = "Fixture", default)]
    pub(crate) fixtures: Vec<Fixture>,
    // FIXME: #[serde(rename = "Support", default)]
    //        supports: Vec<Support>,
    // FIXME: #[serde(rename = "Truss", default)]
    //        trusses: Vec<Truss>,
    // FIXME: #[serde(rename = "VideoScreen", default)]
    //        video_screens: Vec<VideoScreen>,
    // FIXME: #[serde(rename = "Projector", default)]
    //        projectors: Vec<Projector>,
}

impl LayerChildList {
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
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
    fn test_layers_empty() {
        let xml = r#"<Layers/>"#;
        let layers: Layers = quick_xml::de::from_str(xml).unwrap();
        assert!(layers.is_empty());
    }

    #[test]
    fn test_layer_with_fixtures() {
        let xml = r#"
            <Layers>
                <Layer name="Layer 1 Name" uuid="1e247ae7-0f0a-4061-a28d-483e37eb5578">
                    <ChildList>
                        <Fixture name="Robe Robin Esprite" uuid="CE7C4EDA-1C47-4B41-AF56-530116C475B2">
                            <Matrix>{-1.000000,-0.000000,-0.000000}{0.000000,-1.000000,0.000000}{0.000000,0.000000,1.000000}{-1500.000000,-1000.000000,8524.000000}</Matrix>
                            <CustomCommands>
                                <CustomCommand>Blade3Rot.Blade3Rot.Blade3Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade2Rot.Blade2Rot.Blade2Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade4Rot.Blade4Rot.Blade4Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade1Rot.Blade1Rot.Blade1Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade3A.Blade3A.Blade3A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade2A.Blade2A.Blade2A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade4A.Blade4A.Blade4A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade1A.Blade1A.Blade1A,f 0.000000</CustomCommand>
                                <CustomCommand>Yoke_Pan.Pan.Pan 1,f 0.000000</CustomCommand>
                                <CustomCommand>Body_Tilt.Tilt.Tilt 1,f 0.000000</CustomCommand>
                                <CustomCommand>Base_Zoom.Zoom.Zoom 1,f 5.500000</CustomCommand>
                            </CustomCommands>
                            <Classing>4157C914-094B-4808-87EE-DD7EBD6F9F97</Classing>
                            <GDTFSpec>Robe Lighting@Robin Esprite.gdtf</GDTFSpec>
                            <GDTFMode>Mode 1 - Standard 16 bit</GDTFMode>
                            <Addresses>
                                <Address break="0">20993</Address>
                            </Addresses>
                            <FixtureID>1001</FixtureID>
                            <UnitNumber>0</UnitNumber>
                            <FixtureTypeId>0</FixtureTypeId>
                            <CustomId>0</CustomId>
                            <Color>0.312712,0.329008,99.999960</Color>
                            <CastShadow>false</CastShadow>
                            <Mappings />
                        </Fixture>
                        <Fixture name="Robe Robin Esprite" uuid="A0C59C23-869D-48D3-9F46-8B2082A4BFC9">
                            <Matrix>{-1.000000,-0.000000,-0.000000}{0.000000,-1.000000,0.000000}{0.000000,0.000000,1.000000}{0.000000,-1000.000000,8524.000000}</Matrix>
                            <CustomCommands>
                                <CustomCommand>Blade3Rot.Blade3Rot.Blade3Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade2Rot.Blade2Rot.Blade2Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade4Rot.Blade4Rot.Blade4Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade1Rot.Blade1Rot.Blade1Rot,f 0.000000</CustomCommand>
                                <CustomCommand>Blade3A.Blade3A.Blade3A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade2A.Blade2A.Blade2A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade4A.Blade4A.Blade4A,f 0.000000</CustomCommand>
                                <CustomCommand>Blade1A.Blade1A.Blade1A,f 0.000000</CustomCommand>
                                <CustomCommand>Yoke_Pan.Pan.Pan 1,f 0.000000</CustomCommand>
                                <CustomCommand>Body_Tilt.Tilt.Tilt 1,f 0.000000</CustomCommand>
                                <CustomCommand>Base_Zoom.Zoom.Zoom 1,f 5.500000</CustomCommand>
                            </CustomCommands>
                            <Classing>4157C914-094B-4808-87EE-DD7EBD6F9F97</Classing>
                            <GDTFSpec>Robe Lighting@Robin Esprite.gdtf</GDTFSpec>
                            <GDTFMode>Mode 1 - Standard 16 bit</GDTFMode>
                            <Addresses>
                                <Address break="0">21042</Address>
                            </Addresses>
                            <FixtureID>1002</FixtureID>
                            <UnitNumber>0</UnitNumber>
                            <FixtureTypeId>0</FixtureTypeId>
                            <CustomId>0</CustomId>
                            <Color>0.312712,0.329008,99.999960</Color>
                            <CastShadow>false</CastShadow>
                            <Mappings />
                        </Fixture>
                    </ChildList>
                </Layer>
            </Layers>
        "#;
        let layers: Layers = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(layers.len(), 1);
        let layer = &layers[0];
        assert_eq!(layer.name(), "Layer 1 Name");
        assert_eq!(layer.uuid(), uuid("1e247ae7-0f0a-4061-a28d-483e37eb5578"));
        assert!(layer.matrix().is_none());
        assert_eq!(layer.child_list.fixtures.len(), 2);

        let fixture1 = &layer.child_list.fixtures[0];
        assert_eq!(fixture1.name, "Robe Robin Esprite");
        assert_eq!(fixture1.uuid, uuid("CE7C4EDA-1C47-4B41-AF56-530116C475B2"));
        assert!(fixture1.matrix.is_some());
        assert_eq!(fixture1.fixture_id.as_deref(), Some("1001"));

        let fixture2 = &layer.child_list.fixtures[1];
        assert_eq!(fixture2.name, "Robe Robin Esprite");
        assert_eq!(fixture2.uuid, uuid("A0C59C23-869D-48D3-9F46-8B2082A4BFC9"));
        assert!(fixture2.matrix.is_some());
        assert_eq!(fixture2.fixture_id.as_deref(), Some("1002"));
    }

    #[test]
    fn test_layer_name_optional() {
        let xml = r#"<Layer uuid="1e247ae7-0f0a-4061-a28d-483e37eb5578"/>"#;
        let layer: Layer = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(layer.uuid(), uuid("1e247ae7-0f0a-4061-a28d-483e37eb5578"));
        assert_eq!(layer.name(), "");
        assert!(layer.matrix().is_none());
        assert!(layer.child_list.fixtures.is_empty());
    }

    #[test]
    fn test_layer_matrix_optional() {
        let xml = r#"
            <Layer uuid="1e247ae7-0f0a-4061-a28d-483e37eb5578" name="Layer X">
                <ChildList/>
            </Layer>
        "#;
        let layer: Layer = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(layer.name(), "Layer X");
        assert!(layer.matrix().is_none());
    }
}
