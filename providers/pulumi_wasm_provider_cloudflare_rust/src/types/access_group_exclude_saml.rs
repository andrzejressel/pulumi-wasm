#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessGroupExcludeSaml {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of the Azure Identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
