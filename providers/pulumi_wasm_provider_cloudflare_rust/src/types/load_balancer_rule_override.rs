#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverride {
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideAdaptiveRouting>>>,
    #[serde(rename = "countryPools")]
    pub r#country_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideCountryPool>>>,
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Box<Option<Vec<String>>>,
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Box<Option<String>>,
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideLocationStrategy>>>,
    #[serde(rename = "popPools")]
    pub r#pop_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverridePopPool>>>,
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRandomSteering>>>,
    #[serde(rename = "regionPools")]
    pub r#region_pools: Box<Option<Vec<crate::types::LoadBalancerRuleOverrideRegionPool>>>,
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<String>>,
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes:
        Box<Option<Vec<crate::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>>,
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Box<Option<i32>>,
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Box<Option<String>>,
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}
