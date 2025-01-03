#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig {
    /// Whether Amazon AppFlow aggregates the flow records into a single file, or leave them unaggregated. Valid values are `None` and `SingleFile`.
    #[builder(into, default)]
    #[serde(rename = "aggregationType")]
    pub r#aggregation_type: Box<Option<String>>,
    /// The desired file size, in MB, for each output file that Amazon AppFlow writes to the flow destination. Integer value.
    #[builder(into, default)]
    #[serde(rename = "targetFileSize")]
    pub r#target_file_size: Box<Option<i32>>,
}
