#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorExplicitResourceOrder {
    #[builder(into, default)]
    #[serde(rename = "backendPoolHealthProbeIds")]
    pub r#backend_pool_health_probe_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "backendPoolIds")]
    pub r#backend_pool_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "backendPoolLoadBalancingIds")]
    pub r#backend_pool_load_balancing_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "frontendEndpointIds")]
    pub r#frontend_endpoint_ids: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "routingRuleIds")]
    pub r#routing_rule_ids: Box<Option<Vec<String>>>,
}
