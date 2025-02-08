#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerRuleOverride {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    #[builder(into, default)]
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings: Box<Option<Vec<super::types::LoadBalancerRuleOverrideAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    #[builder(into, default)]
    #[serde(rename = "countryPools")]
    pub r#country_pools: Box<Option<Vec<super::types::LoadBalancerRuleOverrideCountryPool>>>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    #[builder(into, default)]
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Box<Option<Vec<String>>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    #[builder(into, default)]
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Box<Option<String>>,
    /// Controls location-based steering for non-proxied requests.
    #[builder(into, default)]
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies: Box<Option<Vec<super::types::LoadBalancerRuleOverrideLocationStrategy>>>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    #[builder(into, default)]
    #[serde(rename = "popPools")]
    pub r#pop_pools: Box<Option<Vec<super::types::LoadBalancerRuleOverridePopPool>>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    #[builder(into, default)]
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Box<Option<Vec<super::types::LoadBalancerRuleOverrideRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    #[builder(into, default)]
    #[serde(rename = "regionPools")]
    pub r#region_pools: Box<Option<Vec<super::types::LoadBalancerRuleOverrideRegionPool>>>,
    /// Configure attributes for session affinity.
    #[builder(into, default)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<String>>,
    /// Configure attributes for session affinity. Note that the property `drain_duration` is not currently supported as a rule override.
    #[builder(into, default)]
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes: Box<Option<Vec<super::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    #[builder(into, default)]
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Box<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    #[builder(into, default)]
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Box<Option<String>>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}
