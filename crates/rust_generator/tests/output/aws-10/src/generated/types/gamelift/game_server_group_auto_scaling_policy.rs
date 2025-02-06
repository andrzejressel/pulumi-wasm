#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GameServerGroupAutoScalingPolicy {
    /// Length of time, in seconds, it takes for a new instance to start
    /// new game server processes and register with GameLift FleetIQ.
    /// Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up,
    /// because it avoids prematurely starting new instances. Defaults to `60`.
    #[builder(into, default)]
    #[serde(rename = "estimatedInstanceWarmup")]
    pub r#estimated_instance_warmup: Box<Option<i32>>,
    #[builder(into)]
    #[serde(rename = "targetTrackingConfiguration")]
    pub r#target_tracking_configuration: Box<super::super::types::gamelift::GameServerGroupAutoScalingPolicyTargetTrackingConfiguration>,
}
