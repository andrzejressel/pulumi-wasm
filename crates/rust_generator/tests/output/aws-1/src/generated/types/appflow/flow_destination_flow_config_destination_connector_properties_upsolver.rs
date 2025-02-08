#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver {
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "s3OutputFormatConfig")]
    pub r#s_3_output_format_config: Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig>,
}
