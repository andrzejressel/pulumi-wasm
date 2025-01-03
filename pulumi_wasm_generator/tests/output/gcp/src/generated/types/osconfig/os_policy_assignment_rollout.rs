#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentRollout {
    /// The maximum number (or percentage) of VMs
    /// per zone to disrupt at any given moment. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "disruptionBudget")]
    pub r#disruption_budget: Box<super::super::types::osconfig::OsPolicyAssignmentRolloutDisruptionBudget>,
    /// This determines the minimum duration of
    /// time to wait after the configuration changes are applied through the current
    /// rollout. A VM continues to count towards the `disruption_budget` at least
    /// until this duration of time has passed after configuration changes are
    /// applied.
    #[builder(into)]
    #[serde(rename = "minWaitDuration")]
    pub r#min_wait_duration: Box<String>,
}
