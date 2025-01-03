#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRule {
    /// Optional. The `AdvanceRolloutRule` will automatically advance a successful Rollout.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "advanceRolloutRule")]
    pub r#advance_rollout_rule: Box<Option<super::super::types::clouddeploy::AutomationRuleAdvanceRolloutRule>>,
    /// Optional. `PromoteReleaseRule` will automatically promote a release from the current target to a specified target.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "promoteReleaseRule")]
    pub r#promote_release_rule: Box<Option<super::super::types::clouddeploy::AutomationRulePromoteReleaseRule>>,
}
