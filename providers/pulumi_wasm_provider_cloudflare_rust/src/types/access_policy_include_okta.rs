#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyIncludeOkta {
    /// The ID of your Okta identity provider.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<Option<String>>,
    /// The name of the Okta Group.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "names")]
    pub r#names: Box<Option<Vec<String>>>,
}
