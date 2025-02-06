#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomAction {
    /// A configuration block describing the custom action associated with the `action_name`. See Action Definition below for details.
    #[builder(into)]
    #[serde(rename = "actionDefinition")]
    pub r#action_definition: Box<super::super::types::networkfirewall::RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionActionDefinition>,
    /// A friendly name of the custom action.
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: Box<String>,
}
