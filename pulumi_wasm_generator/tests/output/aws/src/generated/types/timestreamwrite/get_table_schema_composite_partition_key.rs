#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableSchemaCompositePartitionKey {
    #[builder(into)]
    #[serde(rename = "enforcementInRecord")]
    pub r#enforcement_in_record: Box<String>,
    /// Name of the Timestream table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Type of partition key.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
