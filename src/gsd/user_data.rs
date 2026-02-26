use facet_xml as xml;

#[derive(facet::Facet, Debug, Clone, PartialEq, Default)]
pub struct UserData {
    /// The data is stored as raw XML markup because its structure may be ambiguous or application-specific.
    /// The user is responsible for parsing or interpreting the contents as needed.
    #[facet(rename = "Data")]
    pub(crate) data: Vec<xml::RawMarkup>,
}

impl UserData {
    pub fn data(&self) -> &[xml::RawMarkup] {
        &self.data
    }
}
