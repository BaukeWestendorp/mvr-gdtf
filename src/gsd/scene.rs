use crate::{AuxData, Layer, Layers};

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Scene {
    #[facet(rename = "AUXData", default)]
    pub(crate) aux_data: AuxData,
    #[facet(rename = "Layers", default)]
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

#[derive(Debug, Clone, PartialEq, facet::Facet)]
pub struct Matrix4x3 {
    pub(crate) u1: f64,
    pub(crate) u2: f64,
    pub(crate) u3: f64,
    pub(crate) v1: f64,
    pub(crate) v2: f64,
    pub(crate) v3: f64,
    pub(crate) w1: f64,
    pub(crate) w2: f64,
    pub(crate) w3: f64,
    pub(crate) o1: f64,
    pub(crate) o2: f64,
    pub(crate) o3: f64,
}
