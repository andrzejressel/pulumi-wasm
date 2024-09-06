#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessGroupIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
