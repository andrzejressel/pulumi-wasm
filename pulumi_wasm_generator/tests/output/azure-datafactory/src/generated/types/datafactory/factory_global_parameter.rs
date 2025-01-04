#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FactoryGlobalParameter {
    /// Specifies the global parameter name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the global parameter type. Possible Values are `Array`, `Bool`, `Float`, `Int`, `Object` or `String`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Specifies the global parameter value.
    /// 
    /// > **Note:** For type `Array` and `Object` it is recommended to use `jsonencode()` for the value
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
