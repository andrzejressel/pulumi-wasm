#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLoggingV2TargetObjectKeyFormatPartitionedPrefix {
    /// Specifies the partition date source for the partitioned prefix. Valid values: `EventTime`, `DeliveryTime`.
    #[builder(into)]
    #[serde(rename = "partitionDateSource")]
    pub r#partition_date_source: Box<String>,
}
