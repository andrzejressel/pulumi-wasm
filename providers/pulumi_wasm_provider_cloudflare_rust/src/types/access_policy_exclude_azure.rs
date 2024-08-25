#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessPolicyExcludeAzure {
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
