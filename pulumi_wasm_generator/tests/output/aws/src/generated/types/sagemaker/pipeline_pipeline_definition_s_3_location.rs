#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelinePipelineDefinitionS3Location {
    /// Name of the S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// The object key (or key name) uniquely identifies the object in an S3 bucket.
    #[builder(into)]
    #[serde(rename = "objectKey")]
    pub r#object_key: Box<String>,
    /// Version Id of the pipeline definition file. If not specified, Amazon SageMaker will retrieve the latest version.
    #[builder(into, default)]
    #[serde(rename = "versionId")]
    pub r#version_id: Box<Option<String>>,
}