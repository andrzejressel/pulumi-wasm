#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig {
    /// File type that Amazon AppFlow gets from your Amazon S3 bucket. Valid values are `CSV` and `JSON`.
    #[builder(into, default)]
    #[serde(rename = "s3InputFileType")]
    pub r#s_3_input_file_type: Box<Option<String>>,
}
