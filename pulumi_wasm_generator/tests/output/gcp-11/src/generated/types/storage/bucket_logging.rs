#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLogging {
    /// The bucket that will receive log objects.
    #[builder(into)]
    #[serde(rename = "logBucket")]
    pub r#log_bucket: Box<String>,
    /// The object prefix for log objects. If it's not provided,
    /// by default GCS sets this to this bucket's name.
    #[builder(into, default)]
    #[serde(rename = "logObjectPrefix")]
    pub r#log_object_prefix: Box<Option<String>>,
}
