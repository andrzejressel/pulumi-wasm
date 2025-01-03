#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionHeader {
    /// Values in the response header with the specified name that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureValues")]
    pub r#failure_values: Box<Vec<String>>,
    /// The name of the header to use.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Values in the response header with the specified name that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successValues")]
    pub r#success_values: Box<Vec<String>>,
}
