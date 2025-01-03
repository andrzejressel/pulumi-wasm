#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableAttribute {
    /// Name of the attribute
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Attribute type. Valid values are `S` (string), `N` (number), `B` (binary).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
