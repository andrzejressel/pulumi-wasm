#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}
