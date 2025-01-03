#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetLogicalTableMapDataTransformRenameColumnOperation {
    /// Column to be renamed.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// New name for the column.
    #[builder(into)]
    #[serde(rename = "newColumnName")]
    pub r#new_column_name: Box<String>,
}
