#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleRuleNoncurrentVersionExpiration {
    /// Specifies the number of days noncurrent object versions expire.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
}