#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCollectionRuleStreamDeclarationColumn {
    /// The name of the column.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of the column data. Possible values are `string`, `int`, `long`, `real`, `boolean`, `datetime`,and `dynamic`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}