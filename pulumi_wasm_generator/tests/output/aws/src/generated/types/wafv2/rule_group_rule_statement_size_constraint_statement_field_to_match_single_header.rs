#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementSizeConstraintStatementFieldToMatchSingleHeader {
    /// The name of the query header to inspect. This setting must be provided as lower case characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
