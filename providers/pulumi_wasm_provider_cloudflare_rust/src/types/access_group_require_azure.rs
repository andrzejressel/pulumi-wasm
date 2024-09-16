#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessGroupRequireAzure {
    /// The ID of the Azure Identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Authentication Context.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
