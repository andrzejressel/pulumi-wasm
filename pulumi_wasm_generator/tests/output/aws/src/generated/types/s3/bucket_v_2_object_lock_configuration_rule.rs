#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2ObjectLockConfigurationRule {
    /// Default retention period that you want to apply to new objects placed in this bucket (documented below).
    #[builder(into)]
    #[serde(rename = "defaultRetentions")]
    pub r#default_retentions: Box<Vec<super::super::types::s3::BucketV2ObjectLockConfigurationRuleDefaultRetention>>,
}