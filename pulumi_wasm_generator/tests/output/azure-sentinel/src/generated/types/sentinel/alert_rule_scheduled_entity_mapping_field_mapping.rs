#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertRuleScheduledEntityMappingFieldMapping {
    /// The column name to be mapped to the identifier.
    #[builder(into)]
    #[serde(rename = "columnName")]
    pub r#column_name: Box<String>,
    /// The identifier of the entity.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
}