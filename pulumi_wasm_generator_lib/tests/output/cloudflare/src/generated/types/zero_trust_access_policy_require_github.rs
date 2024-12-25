#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessPolicyRequireGithub {
    /// The ID of your Github identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The name of the organization.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The teams that should be matched.
    #[builder(into, default)]
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}
