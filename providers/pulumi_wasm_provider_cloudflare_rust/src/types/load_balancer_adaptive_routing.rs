#[derive(serde::Serialize)]
pub struct LoadBalancerAdaptiveRouting {
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}
