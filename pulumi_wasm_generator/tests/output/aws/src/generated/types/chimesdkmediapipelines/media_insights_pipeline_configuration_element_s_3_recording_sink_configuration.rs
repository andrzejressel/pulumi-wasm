#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration {
    /// S3 URI to deliver recordings.
    #[builder(into, default)]
    #[serde(rename = "destination")]
    pub r#destination: Box<Option<String>>,
}