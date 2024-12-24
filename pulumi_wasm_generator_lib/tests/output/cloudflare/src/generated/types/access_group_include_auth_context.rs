#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessGroupIncludeAuthContext {
    /// The ACID of the Authentication Context.
    #[builder(into)]
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure identity provider.
    #[builder(into)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}
