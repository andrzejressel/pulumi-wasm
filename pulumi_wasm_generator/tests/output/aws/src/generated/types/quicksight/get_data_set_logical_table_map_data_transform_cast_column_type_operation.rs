#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetLogicalTableMapDataTransformCastColumnTypeOperation {
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    #[builder(into)]
    #[serde(rename = "newColumnType")]
    pub r#new_column_type: Box<String>,
}