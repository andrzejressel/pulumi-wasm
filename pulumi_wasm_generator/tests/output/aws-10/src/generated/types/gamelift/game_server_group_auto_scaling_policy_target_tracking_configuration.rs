#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GameServerGroupAutoScalingPolicyTargetTrackingConfiguration {
    /// Desired value to use with a game server group target-based scaling policy.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
