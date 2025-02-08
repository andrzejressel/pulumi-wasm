#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowLogDestinationOptions {
    /// The format for the flow log. Default value: `plain-text`. Valid values: `plain-text`, `parquet`.
    #[builder(into, default)]
    #[serde(rename = "fileFormat")]
    pub r#file_format: Box<Option<String>>,
    /// Indicates whether to use Hive-compatible prefixes for flow logs stored in Amazon S3. Default value: `false`.
    #[builder(into, default)]
    #[serde(rename = "hiveCompatiblePartitions")]
    pub r#hive_compatible_partitions: Box<Option<bool>>,
    /// Indicates whether to partition the flow log per hour. This reduces the cost and response time for queries. Default value: `false`.
    #[builder(into, default)]
    #[serde(rename = "perHourPartition")]
    pub r#per_hour_partition: Box<Option<bool>>,
}
