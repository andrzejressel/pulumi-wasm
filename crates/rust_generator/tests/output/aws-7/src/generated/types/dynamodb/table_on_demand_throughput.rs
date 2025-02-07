#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableOnDemandThroughput {
    /// Maximum number of read request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into, default)]
    #[serde(rename = "maxReadRequestUnits")]
    pub r#max_read_request_units: Box<Option<i32>>,
    /// Maximum number of write request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into, default)]
    #[serde(rename = "maxWriteRequestUnits")]
    pub r#max_write_request_units: Box<Option<i32>>,
}
