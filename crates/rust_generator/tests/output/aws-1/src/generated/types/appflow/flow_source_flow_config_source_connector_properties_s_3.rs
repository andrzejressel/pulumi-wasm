#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesS3 {
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<String>,
    /// When you use Amazon S3 as the source, the configuration format that you provide the flow input data. See S3 Input Format Config for details.
    #[builder(into, default)]
    #[serde(rename = "s3InputFormatConfig")]
    pub r#s_3_input_format_config: Box<Option<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorPropertiesS3S3InputFormatConfig>>,
}
