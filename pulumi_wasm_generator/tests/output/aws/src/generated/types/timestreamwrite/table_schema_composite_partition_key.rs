#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableSchemaCompositePartitionKey {
    /// The level of enforcement for the specification of a dimension key in ingested records. Valid values: `REQUIRED`, `OPTIONAL`.
    #[builder(into, default)]
    #[serde(rename = "enforcementInRecord")]
    pub r#enforcement_in_record: Box<Option<String>>,
    /// The name of the attribute used for a dimension key.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type of the partition key. Valid values: `DIMENSION`, `MEASURE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}