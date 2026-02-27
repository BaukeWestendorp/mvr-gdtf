use crate::{AuxData, Layer, Layers};

#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Scene")]
pub struct Scene {
    #[serde(rename = "AUXData", default)]
    pub(crate) aux_data: AuxData,
    #[serde(rename = "Layers", default)]
    pub(crate) layers: Layers,
}

impl Scene {
    pub fn aux_data(&self) -> &AuxData {
        &self.aux_data
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_deserialize() {
        let xml = r#"
            <Scene>
                <AUXData></AUXData>
                <Layers></Layers>
            </Scene>
        "#;
        let scene: Scene = quick_xml::de::from_str(xml).expect("Should deserialize");
        assert_eq!(scene.aux_data(), &AuxData::default());
        assert_eq!(scene.layers().len(), 0);
    }
}
