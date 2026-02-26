use std::str::FromStr;

use facet::Opaque;
use uuid::Uuid;

#[derive(facet::Facet, Debug, Clone)]
#[facet(rename = "GeneralSceneDescription")]
pub struct GeneralSceneDescription {
    #[facet(facet_xml::attribute, rename = "verMajor")]
    ver_major: i32,
    #[facet(facet_xml::attribute, rename = "verMinor")]
    ver_minor: i32,
    #[facet(facet_xml::attribute, rename = "provider")]
    provider: String,
    #[facet(facet_xml::attribute, rename = "providerVersion")]
    provider_version: String,

    #[facet(rename = "UserData")]
    user_data: Option<UserData>,
    #[facet(rename = "Scene")]
    scene: Scene,
}

impl GeneralSceneDescription {
    pub fn ver_major(&self) -> i32 {
        self.ver_major
    }

    pub fn ver_minor(&self) -> i32 {
        self.ver_minor
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn provider_version(&self) -> &str {
        &self.provider_version
    }

    pub fn user_data(&self) -> Option<&UserData> {
        self.user_data.as_ref()
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct UserData {
    /// The data is stored as raw XML markup because its structure may be ambiguous or application-specific.
    /// The user is responsible for parsing or interpreting the contents as needed.
    #[facet(rename = "Data")]
    data: Vec<facet_xml::RawMarkup>,
}

impl UserData {
    pub fn data(&self) -> &[facet_xml::RawMarkup] {
        &self.data
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Scene {
    #[facet(rename = "AUXData")]
    aux_data: Option<AuxData>,
    #[facet(rename = "Layers", default)]
    layers: Layers,
}

impl Scene {
    pub fn aux_data(&self) -> Option<&AuxData> {
        self.aux_data.as_ref()
    }

    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct AuxData {
    #[facet(rename = "Symdef")]
    symdefs: Vec<Symdef>,
    #[facet(rename = "Position")]
    positions: Vec<Position>,
    #[facet(rename = "MappingDefinition")]
    mapping_definitions: Vec<MappingDefinition>,
    #[facet(rename = "Class")]
    classes: Vec<Class>,
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
        &self.classes
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Class {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
}

impl Class {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Position {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
}

impl Position {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Symdef {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
    #[facet(untagged)]
    content: SymdefContent,
}

impl Symdef {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &SymdefContent {
        &self.content
    }
}

#[derive(facet::Facet, Debug, Clone)]
#[repr(u8)]
pub enum SymdefContent {
    Geometry3D(Geometry3D),
    Symbol(Symbol),
}

#[derive(facet::Facet, Debug, Clone)]
pub struct MappingDefinition {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
    //
    // FIXME: Add children
}

impl MappingDefinition {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Layers {
    #[facet(rename = "Layer")]
    layers: Vec<Layer>,
}

impl std::ops::Deref for Layers {
    type Target = [Layer];

    fn deref(&self) -> &Self::Target {
        &self.layers
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Layer {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
    #[facet(rename = "Matrix")]
    matrix: Option<Matrix>,
}

impl Layer {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn matrix(&self) -> Option<&Matrix> {
        self.matrix.as_ref()
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Geometry3D {
    #[facet(facet_xml::attribute, rename = "fileName")]
    file_name: String,

    #[facet(opaque, deserialize_with = opaque_type_from_str)]
    matrix: Matrix,
}

impl Geometry3D {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

#[derive(facet::Facet, Debug, Clone)]
pub struct Symbol {
    #[facet(facet_xml::attribute, rename = "uuid")]
    uuid: Uuid,
    #[facet(facet_xml::attribute, rename = "name", default = "")]
    name: String,
}

impl Symbol {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    u1: f64,
    u2: f64,
    u3: f64,
    v1: f64,
    v2: f64,
    v3: f64,
    w1: f64,
    w2: f64,
    w3: f64,
    o1: f64,
    o2: f64,
    o3: f64,
}

impl FromStr for Matrix {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(char::is_whitespace, "");

        let mut values = Vec::new();
        let mut rest = s.as_str();
        while let Some(start) = rest.find('{') {
            let rest2 = &rest[start + 1..];
            if let Some(end) = rest2.find('}') {
                let group = &rest2[..end];
                for num in group.split(',') {
                    if num.is_empty() {
                        return Err(crate::Error::MatrixParseError(
                            "Empty value in matrix group".to_string(),
                        ));
                    }
                    let val: f64 = num.parse().map_err(|_| {
                        crate::Error::MatrixParseError(format!("Failed to parse '{}' as f64", num))
                    })?;
                    values.push(val);
                }
                rest = &rest2[end + 1..];
            } else {
                return Err(crate::Error::MatrixParseError(
                    "Mismatched '{' in matrix string".to_string(),
                ));
            }
        }

        if values.len() != 12 {
            return Err(crate::Error::MatrixParseError(format!(
                "Expected 12 values for Matrix, got {}",
                values.len()
            )));
        }

        Ok(Matrix {
            u1: values[0],
            u2: values[1],
            u3: values[2],
            v1: values[3],
            v2: values[4],
            v3: values[5],
            w1: values[6],
            w2: values[7],
            w3: values[8],
            o1: values[9],
            o2: values[10],
            o3: values[11],
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_parse_matrix() {
        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12}";
        let m = Matrix::from_str(s).unwrap();
        assert_eq!(m.u1, 1.0);
        assert_eq!(m.u2, 2.0);
        assert_eq!(m.u3, 3.0);
        assert_eq!(m.v1, 4.0);
        assert_eq!(m.v2, 5.0);
        assert_eq!(m.v3, 6.0);
        assert_eq!(m.w1, 7.0);
        assert_eq!(m.w2, 8.0);
        assert_eq!(m.w3, 9.0);
        assert_eq!(m.o1, 10.0);
        assert_eq!(m.o2, 11.0);
        assert_eq!(m.o3, 12.0);

        let s = " { 1 , 2 , 3 } { 4 , 5 , 6 } { 7 , 8 , 9 } { 10 , 11 , 12 } ";
        let m = Matrix::from_str(s).unwrap();
        assert_eq!(m.u1, 1.0);
        assert_eq!(m.u2, 2.0);
        assert_eq!(m.u3, 3.0);
        assert_eq!(m.v1, 4.0);
        assert_eq!(m.v2, 5.0);
        assert_eq!(m.v3, 6.0);
        assert_eq!(m.w1, 7.0);
        assert_eq!(m.w2, 8.0);
        assert_eq!(m.w3, 9.0);
        assert_eq!(m.o1, 10.0);
        assert_eq!(m.o2, 11.0);
        assert_eq!(m.o3, 12.0);

        let s = "{1,2,3}{4,5,6}{7,8,9}";
        let err = Matrix::from_str(s).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Expected 12 values"));

        let s = "{1,2,3}{4,5,6}{7,8,9}{10,11,12,13}";
        let err = Matrix::from_str(s).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Expected 12 values"));

        let s = "{1,2,foo}{4,5,6}{7,8,9}{10,11,12}";
        let err = Matrix::from_str(s).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Failed to parse"));

        let s = "{1,2,3}{4,5,6}{7,8,9{10,11,12}";
        let err = Matrix::from_str(s).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("Mismatched '{'"));
    }
}
