#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2Logging {
    /// Name of the bucket that will receive the log objects.
    #[builder(into)]
    #[serde(rename = "targetBucket")]
    pub r#target_bucket: Box<String>,
    /// To specify a key prefix for log objects.
    #[builder(into, default)]
    #[serde(rename = "targetPrefix")]
    pub r#target_prefix: Box<Option<String>>,
}