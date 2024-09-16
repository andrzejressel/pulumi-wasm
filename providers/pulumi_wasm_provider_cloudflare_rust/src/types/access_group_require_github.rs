#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessGroupRequireGithub {
    /// The ID of the Azure Identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "teams")]
    pub r#teams: Box<Option<Vec<String>>>,
}
