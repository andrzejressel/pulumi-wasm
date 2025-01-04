#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationFeatureTargetingFilter {
    /// A number representing the percentage of the entire user base.
    #[builder(into)]
    #[serde(rename = "defaultRolloutPercentage")]
    pub r#default_rollout_percentage: Box<i32>,
    /// One or more `groups` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<super::super::types::appconfiguration::ConfigurationFeatureTargetingFilterGroup>>>,
    /// A list of users to target for this feature.
    #[builder(into, default)]
    #[serde(rename = "users")]
    pub r#users: Box<Option<Vec<String>>>,
}
