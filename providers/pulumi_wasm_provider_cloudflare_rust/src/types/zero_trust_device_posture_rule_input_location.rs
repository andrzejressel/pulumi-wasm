#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDevicePostureRuleInputLocation {
    /// List of paths to check for client certificate rule.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "paths")]
    pub r#paths: Box<Option<Vec<String>>>,
    /// List of trust stores to check for client certificate rule. Available values: `system`, `user`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "trustStores")]
    pub r#trust_stores: Box<Option<Vec<String>>>,
}
