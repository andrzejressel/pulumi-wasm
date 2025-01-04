#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetResourceCreationLimitPolicy {
    /// Maximum number of game sessions that an individual can create during the policy period.
    #[builder(into, default)]
    #[serde(rename = "newGameSessionsPerCreator")]
    pub r#new_game_sessions_per_creator: Box<Option<i32>>,
    /// Time span used in evaluating the resource creation limit policy.
    #[builder(into, default)]
    #[serde(rename = "policyPeriodInMinutes")]
    pub r#policy_period_in_minutes: Box<Option<i32>>,
}
