#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessApplicationScimConfigMapping {
    /// Whether or not this mapping is enabled.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// A [SCIM filter expression](https://datatracker.ietf.org/doc/html/rfc7644#section-3.4.2.2) that matches resources that should be provisioned to this application.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<String>>,
    /// Whether or not this mapping applies to creates, updates, or deletes.
    #[builder(into, default)]
    #[serde(rename = "operations")]
    pub r#operations: Box<Option<super::types::ZeroTrustAccessApplicationScimConfigMappingOperations>>,
    /// Which SCIM resource type this mapping applies to.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
    /// A [JSONata](https://jsonata.org/) expression that transforms the resource before provisioning it in the application.
    #[builder(into, default)]
    #[serde(rename = "transformJsonata")]
    pub r#transform_jsonata: Box<Option<String>>,
}
