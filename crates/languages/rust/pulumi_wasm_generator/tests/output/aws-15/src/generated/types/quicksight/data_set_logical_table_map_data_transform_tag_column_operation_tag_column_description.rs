#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetLogicalTableMapDataTransformTagColumnOperationTagColumnDescription {
    /// The text of a description for a column.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<String>>,
}
