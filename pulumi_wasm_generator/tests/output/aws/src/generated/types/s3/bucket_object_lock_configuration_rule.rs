#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketObjectLockConfigurationRule {
    /// The default retention period that you want to apply to new objects placed in this bucket.
    #[builder(into)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Box<super::super::types::s3::BucketObjectLockConfigurationRuleDefaultRetention>,
}
