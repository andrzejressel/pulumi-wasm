#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig {
    /// Aggregation settings that you can use to customize the output format of your flow data. See Aggregation Config for more details.
    #[builder(into, default)]
    #[serde(rename = "aggregationConfig")]
    pub r#aggregation_config: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig>>,
    /// File type that Amazon AppFlow places in the Amazon S3 bucket. Valid values are `CSV`, `JSON`, and `PARQUET`.
    #[builder(into, default)]
    #[serde(rename = "fileType")]
    pub r#file_type: Box<Option<String>>,
    /// Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date. See Prefix Config for more details.
    #[builder(into)]
    #[serde(rename = "prefixConfig")]
    pub r#prefix_config: Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig>,
}
