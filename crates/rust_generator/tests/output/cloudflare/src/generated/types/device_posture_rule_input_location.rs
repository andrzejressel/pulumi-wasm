#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DevicePostureRuleInputLocation {
    /// List of paths to check for client certificate rule.
    #[builder(into, default)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Option<Vec<String>>>,
    /// List of trust stores to check for client certificate rule. Available values: `system`, `user`.
    #[builder(into, default)]
    #[serde(rename = "trustStores")]
    pub r#trust_stores: Box<Option<Vec<String>>>,
}
