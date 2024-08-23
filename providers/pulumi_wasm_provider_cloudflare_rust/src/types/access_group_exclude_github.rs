#[derive(serde::Serialize)]
pub struct AccessGroupExcludeGithub {
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}
