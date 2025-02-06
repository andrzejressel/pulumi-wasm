#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementAndStatement {
    /// The statements to combine.
    #[builder(into)]
    #[serde(rename = "statements")]
    pub r#statements: Box<Vec<super::super::types::wafv2::RuleGroupRuleStatement>>,
}
