#[derive(serde::Serialize)]
pub struct AccessGroupIncludeGsuite {
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
}
