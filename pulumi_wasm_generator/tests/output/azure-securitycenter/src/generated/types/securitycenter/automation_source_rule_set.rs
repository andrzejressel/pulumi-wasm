#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationSourceRuleSet {
    /// One or more `rule` blocks as defined below.
    /// 
    /// > **NOTE:** This automation will trigger when all of the `rule`s in this `rule_set` are evaluated as 'true'. This is equivalent to a logical 'AND'.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::securitycenter::AutomationSourceRuleSetRule>>,
}
