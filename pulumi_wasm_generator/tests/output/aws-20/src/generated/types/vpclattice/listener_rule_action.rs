#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleAction {
    /// Describes the rule action that returns a custom HTTP response.
    #[builder(into, default)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<super::super::types::vpclattice::ListenerRuleActionFixedResponse>>,
    /// The forward action. Traffic that matches the rule is forwarded to the specified target groups.
    #[builder(into, default)]
    #[serde(rename = "forward")]
    pub r#forward: Box<Option<super::super::types::vpclattice::ListenerRuleActionForward>>,
}
