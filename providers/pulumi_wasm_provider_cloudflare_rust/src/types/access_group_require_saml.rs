#[derive(serde::Serialize)]
pub struct AccessGroupRequireSaml {
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
