#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetGroupTargetGroupHealth {
    /// Block to configure DNS Failover requirements. See DNS Failover below for details on attributes.
    #[builder(into, default)]
    #[serde(rename = "dnsFailover")]
    pub r#dns_failover: Box<Option<super::super::types::alb::TargetGroupTargetGroupHealthDnsFailover>>,
    /// Block to configure Unhealthy State Routing requirements. See Unhealthy State Routing below for details on attributes.
    #[builder(into, default)]
    #[serde(rename = "unhealthyStateRouting")]
    pub r#unhealthy_state_routing: Box<Option<super::super::types::alb::TargetGroupTargetGroupHealthUnhealthyStateRouting>>,
}
