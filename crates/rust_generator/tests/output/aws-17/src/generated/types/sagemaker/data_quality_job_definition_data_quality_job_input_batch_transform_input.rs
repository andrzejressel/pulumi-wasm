#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataQualityJobDefinitionDataQualityJobInputBatchTransformInput {
    /// The Amazon S3 location being used to capture the data.
    #[builder(into)]
    #[serde(rename = "dataCapturedDestinationS3Uri")]
    pub r#data_captured_destination_s_3_uri: Box<String>,
    /// The dataset format for your batch transform job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "datasetFormat")]
    pub r#dataset_format: Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormat>,
    /// Path to the filesystem where the batch transform data is available to the container. Defaults to `/opt/ml/processing/input`.
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
