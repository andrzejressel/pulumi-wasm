#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclDefaultAction {
    /// Specifies how you want AWS WAF Regional to respond to requests that match the settings in a ruleE.g., `ALLOW`, `BLOCK` or `COUNT`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}