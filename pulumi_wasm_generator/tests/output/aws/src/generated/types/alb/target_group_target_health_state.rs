#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetGroupTargetHealthState {
    /// Indicates whether the load balancer terminates connections to unhealthy targets. Possible values are `true` or `false`. Default: `true`.
    #[builder(into)]
    #[serde(rename = "enableUnhealthyConnectionTermination")]
    pub r#enable_unhealthy_connection_termination: Box<bool>,
    /// Indicates the time to wait for in-flight requests to complete when a target becomes unhealthy. The range is `0-360000`. This value has to be set only if `enable_unhealthy_connection_termination` is set to false. Default: `0`.
    #[builder(into, default)]
    #[serde(rename = "unhealthyDrainingInterval")]
    pub r#unhealthy_draining_interval: Box<Option<i32>>,
}