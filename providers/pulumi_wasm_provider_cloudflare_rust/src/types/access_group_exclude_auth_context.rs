#[derive(serde::Serialize)]
pub struct AccessGroupExcludeAuthContext {
    /// The ACID of the Authentication Context.
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure Identity provider.
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}
