#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerRuleActionForwardStickiness {
    /// The time period, in seconds, during which requests from a client should be routed to the same target group.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<f64>,
    /// Indicates whether target group stickiness is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}