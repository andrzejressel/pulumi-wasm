#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EmailRoutingRuleMatcher {
    /// Field to match on. Required for `type` of `literal`.
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// Type of matcher. Available values: `literal`, `all`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Value to match on. Required for `type` of `literal`.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
