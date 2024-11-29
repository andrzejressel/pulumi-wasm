#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyRequireSaml {
    /// The name of the SAML attribute.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    /// The SAML attribute value to look for.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of your SAML identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
