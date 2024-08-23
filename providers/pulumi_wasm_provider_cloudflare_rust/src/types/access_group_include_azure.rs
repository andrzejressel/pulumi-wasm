#[derive(serde::Serialize)]
pub struct AccessGroupIncludeAzure {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
