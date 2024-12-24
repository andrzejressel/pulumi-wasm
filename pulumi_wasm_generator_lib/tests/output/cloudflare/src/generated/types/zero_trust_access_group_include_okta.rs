#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessGroupIncludeOkta {
    /// The ID of your Okta identity provider.
    #[builder(into, default)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The name of the Okta Group.
    #[builder(into, default)]
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}
