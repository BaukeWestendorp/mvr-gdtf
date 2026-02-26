use std::{ops, str::FromStr as _};

use facet_xml as xml;
use uuid::Uuid;

use crate::Matrix4x3;

#[derive(facet::Facet, Debug, Clone, PartialEq)]
pub struct Layers {
    #[facet(rename = "Layer")]
    pub(crate) layers: Vec<Layer>,
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
    pub(crate) uuid: Uuid,
    #[facet(xml::attribute, rename = "name", default = "")]
    pub(crate) name: String,

    // FIXME: Find a way to serialize the Matrix directly using facet.
    #[facet(flatten, rename = "Matrix")]
    pub(crate) matrix: Option<String>,

    #[facet(rename = "ChildList", default = LayerChildList::default())]
    pub(crate) child_list: LayerChildList,
}

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct LayerChildList {
    // FIXME: #[facet(rename = "SceneObject")]
    //        scene_objects: Vec<SceneObject>,
    // FIXME: #[facet(rename = "GroupObject")]
    //        group_objects: Vec<GroupObject>,
    // FIXME: #[facet(rename = "FocusPoint")]
    //        focus_points: Vec<FocusPoint>,
    // FIXME: #[facet(rename = "Fixture")]
    //        fixtures: Vec<Fixture>,
    // FIXME: #[facet(rename = "Support")]
    //        supports: Vec<Support>,
    // FIXME: #[facet(rename = "Truss")]
    //        trusses: Vec<Truss>,
    // FIXME: #[facet(rename = "VideoScreen")]
    //        video_screens: Vec<VideoScreen>,
    // FIXME: #[facet(rename = "Projector")]
    //        projectors: Vec<Projector>,
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
            .as_ref()
            .and_then(|s| Matrix4x3::from_str(s).ok())
    }
}
