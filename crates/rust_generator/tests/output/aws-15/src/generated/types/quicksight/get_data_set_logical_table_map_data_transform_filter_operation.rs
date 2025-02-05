#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetLogicalTableMapDataTransformFilterOperation {
    #[builder(into)]
    #[serde(rename = "conditionExpression")]
    pub r#condition_expression: Box<String>,
}
