#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountQueuePropertiesMinuteMetrics {
    /// Indicates whether metrics should generate summary statistics for called API operations.
    #[builder(into, default)]
    #[serde(rename = "includeApis")]
    pub r#include_apis: Box<Option<bool>>,
    /// Specifies the number of days that logs will be retained.
    #[builder(into, default)]
    #[serde(rename = "retentionPolicyDays")]
    pub r#retention_policy_days: Box<Option<i32>>,
    /// The version of storage analytics to configure.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}