#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayWafConfigurationDisabledRuleGroup {
    /// The rule group where specific rules are disabled.
    #[builder(into)]
    #[serde(rename = "ruleGroupName")]
    pub r#rule_group_name: Box<String>,
    /// A list of rules which will be disabled in that group.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<i32>>,
}
