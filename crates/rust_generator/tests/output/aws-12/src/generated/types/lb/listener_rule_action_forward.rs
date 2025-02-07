#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleActionForward {
    /// The target group stickiness for the rule.
    #[builder(into, default)]
    #[serde(rename = "stickiness")]
    pub r#stickiness: Box<Option<super::super::types::lb::ListenerRuleActionForwardStickiness>>,
    /// One or more target group blocks.
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Vec<super::super::types::lb::ListenerRuleActionForwardTargetGroup>>,
}
