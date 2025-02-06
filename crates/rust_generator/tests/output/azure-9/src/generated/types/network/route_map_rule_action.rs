#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteMapRuleAction {
    /// A `parameter` block as defined below. Required if `type` is anything other than `Drop`.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::network::RouteMapRuleActionParameter>>>,
    /// The type of the action to be taken. Possible values are `Add`, `Drop`, `Remove`, `Replace` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
