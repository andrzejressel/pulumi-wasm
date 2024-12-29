#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2ObjectLockConfiguration {
    /// Indicates whether this bucket has an Object Lock configuration enabled. Valid values are `true` or `false`. This argument is not supported in all regions or partitions.
    #[builder(into, default)]
    #[serde(rename = "objectLockEnabled")]
    pub r#object_lock_enabled: Box<Option<String>>,
    /// Object Lock rule in place for this bucket (documented below).
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::super::types::s3::BucketV2ObjectLockConfigurationRule>>>,
}
