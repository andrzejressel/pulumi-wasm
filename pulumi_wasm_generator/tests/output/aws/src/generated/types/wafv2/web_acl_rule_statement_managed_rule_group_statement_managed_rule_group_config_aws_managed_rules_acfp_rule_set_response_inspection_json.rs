#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetResponseInspectionJson {
    /// Values in the response header with the specified name that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureValues")]
    pub r#failure_values: Box<Vec<String>>,
    /// The identifier for the value to match against in the JSON.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// Values in the response header with the specified name that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successValues")]
    pub r#success_values: Box<Vec<String>>,
}