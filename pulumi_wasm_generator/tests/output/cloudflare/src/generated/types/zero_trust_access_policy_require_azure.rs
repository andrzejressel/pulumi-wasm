#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessPolicyRequireAzure {
    /// The ID of the Azure identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Azure group or user.
    #[builder(into, default)]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
