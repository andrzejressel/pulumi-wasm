#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobInputEndpointInput {
    /// An endpoint in customer's account which has `data_capture_config` enabled.
    #[builder(into)]
    #[serde(rename = "endpointName")]
    pub r#endpoint_name: Box<String>,
    /// Path to the filesystem where the endpoint data is available to the container. Defaults to `/opt/ml/processing/input`.
    #[builder(into, default)]
    #[serde(rename = "localPath")]
    pub r#local_path: Box<Option<String>>,
    /// Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key. Defaults to `FullyReplicated`. Valid values are `FullyReplicated` or `ShardedByS3Key`
    #[builder(into, default)]
    #[serde(rename = "s3DataDistributionType")]
    pub r#s_3_data_distribution_type: Box<Option<String>>,
    /// Whether the `Pipe` or `File` is used as the input mode for transferring data for the monitoring job. `Pipe` mode is recommended for large datasets. `File` mode is useful for small files that fit in memory. Defaults to `File`.  Valid values are `Pipe` or `File`
    #[builder(into, default)]
    #[serde(rename = "s3InputMode")]
    pub r#s_3_input_mode: Box<Option<String>>,
}
