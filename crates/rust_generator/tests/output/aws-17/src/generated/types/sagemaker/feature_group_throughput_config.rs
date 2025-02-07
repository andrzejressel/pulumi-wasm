#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureGroupThroughputConfig {
    #[builder(into, default)]
    #[serde(rename = "provisionedReadCapacityUnits")]
    pub r#provisioned_read_capacity_units: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "provisionedWriteCapacityUnits")]
    pub r#provisioned_write_capacity_units: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "throughputMode")]
    pub r#throughput_mode: Box<Option<String>>,
}
