#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustAccessGroupIncludeSaml {
    /// The name of the SAML attribute.
    #[builder(into, default)]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    /// The SAML attribute value to look for.
    #[builder(into, default)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Box<Option<String>>,
    /// The ID of your SAML identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
