#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessGroupRequireGsuite {
    /// The email of the Google Workspace group.
    #[builder(into, default)]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of your Google Workspace identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
