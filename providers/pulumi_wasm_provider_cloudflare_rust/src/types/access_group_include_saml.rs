#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessGroupIncludeSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
