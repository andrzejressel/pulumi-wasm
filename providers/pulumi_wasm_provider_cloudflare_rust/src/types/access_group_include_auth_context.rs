#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAuthContext {
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}
