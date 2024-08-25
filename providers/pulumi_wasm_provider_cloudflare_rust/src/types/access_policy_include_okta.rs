#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessPolicyIncludeOkta {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}
