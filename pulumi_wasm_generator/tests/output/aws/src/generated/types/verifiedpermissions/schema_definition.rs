#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SchemaDefinition {
    /// A JSON string representation of the schema.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}