#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVariablesBool {
    /// The description of the Automation Variable.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Specifies if the Automation Variable is encrypted.
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<bool>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the Automation Variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the Automation Variable.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<bool>,
}