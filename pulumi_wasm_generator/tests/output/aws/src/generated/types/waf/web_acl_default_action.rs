#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclDefaultAction {
    /// Specifies how you want AWS WAF to respond to requests that don't match the criteria in any of the `rules`.
    /// e.g., `ALLOW` or `BLOCK`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}