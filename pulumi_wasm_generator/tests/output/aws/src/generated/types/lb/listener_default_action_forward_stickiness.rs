#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerDefaultActionForwardStickiness {
    /// Time period, in seconds, during which requests from a client should be routed to the same target group. The range is 1-604800 seconds (7 days).
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<i32>,
    /// Whether target group stickiness is enabled. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}