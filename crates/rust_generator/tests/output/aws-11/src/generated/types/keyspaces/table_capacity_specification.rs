#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableCapacitySpecification {
    /// The throughput capacity specified for read operations defined in read capacity units (RCUs).
    #[builder(into, default)]
    #[serde(rename = "readCapacityUnits")]
    pub r#read_capacity_units: Box<Option<i32>>,
    /// The read/write throughput capacity mode for a table. Valid values: `PAY_PER_REQUEST`, `PROVISIONED`. The default value is `PAY_PER_REQUEST`.
    #[builder(into, default)]
    #[serde(rename = "throughputMode")]
    pub r#throughput_mode: Box<Option<String>>,
    /// The throughput capacity specified for write operations defined in write capacity units (WCUs).
    #[builder(into, default)]
    #[serde(rename = "writeCapacityUnits")]
    pub r#write_capacity_units: Box<Option<i32>>,
}
