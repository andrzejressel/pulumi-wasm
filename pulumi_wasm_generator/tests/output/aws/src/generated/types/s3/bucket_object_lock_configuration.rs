#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketObjectLockConfiguration {
    /// Indicates whether this bucket has an Object Lock configuration enabled. Valid value is `Enabled`.
    #[builder(into)]
    #[serde(rename = "objectLockEnabled")]
    pub r#object_lock_enabled: Box<String>,
    /// The Object Lock rule in place for this bucket.
    #[builder(into, default)]
    #[serde(rename = "rule")]
    pub r#rule: Box<Option<super::super::types::s3::BucketObjectLockConfigurationRule>>,
}