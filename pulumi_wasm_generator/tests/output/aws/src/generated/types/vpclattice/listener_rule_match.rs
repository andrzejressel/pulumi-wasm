#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleMatch {
    /// The HTTP criteria that a rule must match.
    #[builder(into, default)]
    #[serde(rename = "httpMatch")]
    pub r#http_match: Box<Option<super::super::types::vpclattice::ListenerRuleMatchHttpMatch>>,
}