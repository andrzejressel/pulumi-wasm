#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomLocationAuthentication {
    /// Specifies the type of authentication.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// Specifies the value of authentication.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}