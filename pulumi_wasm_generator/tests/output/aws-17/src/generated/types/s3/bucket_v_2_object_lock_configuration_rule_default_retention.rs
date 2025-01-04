#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2ObjectLockConfigurationRuleDefaultRetention {
    /// Number of days that you want to specify for the default retention period.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// Default Object Lock retention mode you want to apply to new objects placed in this bucket. Valid values are `GOVERNANCE` and `COMPLIANCE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Number of years that you want to specify for the default retention period.
    #[builder(into, default)]
    #[serde(rename = "years")]
    pub r#years: Box<Option<i32>>,
}
