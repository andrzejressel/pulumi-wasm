#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountQueuePropertiesLogging {
    /// Indicates whether all delete requests should be logged.
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: Box<bool>,
    /// Indicates whether all read requests should be logged.
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: Box<bool>,
    /// Specifies the number of days that logs will be retained.
    #[builder(into, default)]
    #[serde(rename = "retentionPolicyDays")]
    pub r#retention_policy_days: Box<Option<i32>>,
    /// The version of storage analytics to configure.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
    /// Indicates whether all write requests should be logged.
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: Box<bool>,
}
