#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "UserData")]
pub struct UserData {
    #[serde(rename = "Data", default)]
    pub(crate) data: Vec<Data>,
}

impl UserData {
    pub fn data(&self) -> &[Data] {
        &self.data
    }
}

/// NOTE: Because the contents of the `Data` node are ambigous
/// (they're defined by the software producing the MVR), it's non-trivial
/// to parse their content.
#[derive(Debug, Clone, PartialEq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Data")]
pub struct Data {
    #[serde(rename = "@provider")]
    pub(crate) provider: String,
    #[serde(rename = "@ver", default)]
    pub(crate) ver: Option<String>,
}

impl Data {
    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn ver(&self) -> Option<&str> {
        self.ver.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_data_deserialize() {
        let xml_str = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="no" ?>
            <UserData>
                <Data provider="Data Provider 1" ver="0.1" />
                <Data provider="Data Provider 2"><VWEntry key="ce7c4eda-1c47-4b41-af56-530116c475b2">Custom Entry</VWEntry></Data>
            </UserData>
        "#;

        let user_data: UserData = quick_xml::de::from_str(xml_str).expect("Failed to deserialize");
        assert_eq!(user_data.data().len(), 2);

        assert_eq!(user_data.data()[0].provider, "Data Provider 1");
        assert_eq!(user_data.data()[1].ver, None);

        assert_eq!(user_data.data()[1].provider, "Data Provider 2");
        assert_eq!(user_data.data()[1].ver, None);
    }
}
