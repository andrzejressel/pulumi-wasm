#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyRequireAzure {
    /// The ID of the Azure identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The ID of the Azure group or user.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
