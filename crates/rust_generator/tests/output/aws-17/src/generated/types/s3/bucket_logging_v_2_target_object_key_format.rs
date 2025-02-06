#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLoggingV2TargetObjectKeyFormat {
    /// Partitioned S3 key for log objects. See below.
    #[builder(into, default)]
    #[serde(rename = "partitionedPrefix")]
    pub r#partitioned_prefix: Box<Option<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormatPartitionedPrefix>>,
    /// Use the simple format for S3 keys for log objects. To use, set `simple_prefix {}`.
    #[builder(into, default)]
    #[serde(rename = "simplePrefix")]
    pub r#simple_prefix: Box<Option<super::super::types::s3::BucketLoggingV2TargetObjectKeyFormatSimplePrefix>>,
}
