#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoadBalancerRuleOverrideAdaptiveRouting {
    /// Extends zero-downtime failover of requests to healthy origins from alternate pools, when no healthy alternate exists in the same pool, according to the failover order defined by traffic and origin steering. When set `false`, zero-downtime failover will only occur between origins within the same pool. Defaults to `false`.
    #[serde(rename = "failoverAcrossPools")]
    pub r#failover_across_pools: Box<Option<bool>>,
}
