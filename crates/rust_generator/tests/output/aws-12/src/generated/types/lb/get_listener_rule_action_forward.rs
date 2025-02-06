#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerRuleActionForward {
    /// Target group stickiness for the rule.
    /// Detailed below.
    #[builder(into, default)]
    #[serde(rename = "stickiness")]
    pub r#stickiness: Box<Option<super::super::types::lb::GetListenerRuleActionForwardStickiness>>,
    /// Set of target groups for the action.
    /// Detailed below.
    #[builder(into, default)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Box<Option<Vec<super::super::types::lb::GetListenerRuleActionForwardTargetGroup>>>,
}
