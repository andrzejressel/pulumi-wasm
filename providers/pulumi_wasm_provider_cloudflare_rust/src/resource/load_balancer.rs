//! Provides a Cloudflare Load Balancer resource. This sits in front of
//! a number of defined pools of origins and provides various options
//! for geographically-aware load balancing. Note that the load balancing
//! feature must be enabled in your Cloudflare account before you can use
//! this resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = load_balancer::create(
//!         "example",
//!         LoadBalancerArgs::builder()
//!             .country_pools(
//!                 vec![
//!                     LoadBalancerCountryPool::builder().country("US")
//!                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).build_struct(),
//!                 ],
//!             )
//!             .default_pool_ids(vec!["${exampleLoadBalancerPool.id}",])
//!             .description("example load balancer using geo-balancing")
//!             .fallback_pool_id("${exampleLoadBalancerPool.id}")
//!             .name("example-load-balancer.example.com")
//!             .pop_pools(
//!                 vec![
//!                     LoadBalancerPopPool::builder()
//!                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).pop("LAX")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .proxied(true)
//!             .region_pools(
//!                 vec![
//!                     LoadBalancerRegionPool::builder()
//!                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).region("WNAM")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .rules(
//!                 vec![
//!                     LoadBalancerRule::builder()
//!                     .condition("http.request.uri.path contains "testing "")
//!                     .fixedResponse(LoadBalancerRuleFixedResponse::builder()
//!                     .contentType("html").location("www.example.com").messageBody("hello")
//!                     .statusCode(200).build_struct()).name("example rule").build_struct(),
//!                 ],
//!             )
//!             .steering_policy("geo")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let exampleLoadBalancerPool = load_balancer_pool::create(
//!         "exampleLoadBalancerPool",
//!         LoadBalancerPoolArgs::builder()
//!             .name("example-lb-pool")
//!             .origins(
//!                 vec![
//!                     LoadBalancerPoolOrigin::builder().address("192.0.2.1").enabled(false)
//!                     .name("example-1").build_struct(),
//!                 ],
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/loadBalancer:LoadBalancer example <zone_id>/<load_balancer_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerArgs {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub adaptive_routings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    #[builder(into)]
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// Free text description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable or disable the load balancer. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    #[builder(into)]
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    /// Controls location-based steering for non-proxied requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub location_strategies: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub random_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    /// A list of rules for this load balancer to execute.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    /// Configure attributes for session affinity.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity_attributes: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub steering_policy: pulumi_wasm_rust::Output<Option<String>>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct LoadBalancerResult {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    pub adaptive_routings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable or disable the load balancer. Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    /// Controls location-based steering for non-proxied requests.
    pub location_strategies: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
    pub name: pulumi_wasm_rust::Output<String>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    pub random_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    /// A list of rules for this load balancer to execute.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    /// Configure attributes for session affinity.
    pub session_affinity_attributes: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    pub steering_policy: pulumi_wasm_rust::Output<String>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LoadBalancerArgs) -> LoadBalancerResult {

    let result = crate::bindings::pulumi::cloudflare::load_balancer::invoke(name, &crate::bindings::pulumi::cloudflare::load_balancer::Args {
        adaptive_routings: &args.adaptive_routings.get_inner(),
        country_pools: &args.country_pools.get_inner(),
        default_pool_ids: &args.default_pool_ids.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        fallback_pool_id: &args.fallback_pool_id.get_inner(),
        location_strategies: &args.location_strategies.get_inner(),
        name: &args.name.get_inner(),
        pop_pools: &args.pop_pools.get_inner(),
        proxied: &args.proxied.get_inner(),
        random_steerings: &args.random_steerings.get_inner(),
        region_pools: &args.region_pools.get_inner(),
        rules: &args.rules.get_inner(),
        session_affinity: &args.session_affinity.get_inner(),
        session_affinity_attributes: &args.session_affinity_attributes.get_inner(),
        session_affinity_ttl: &args.session_affinity_ttl.get_inner(),
        steering_policy: &args.steering_policy.get_inner(),
        ttl: &args.ttl.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    LoadBalancerResult {
        adaptive_routings: crate::into_domain(result.adaptive_routings),
        country_pools: crate::into_domain(result.country_pools),
        created_on: crate::into_domain(result.created_on),
        default_pool_ids: crate::into_domain(result.default_pool_ids),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        fallback_pool_id: crate::into_domain(result.fallback_pool_id),
        location_strategies: crate::into_domain(result.location_strategies),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        pop_pools: crate::into_domain(result.pop_pools),
        proxied: crate::into_domain(result.proxied),
        random_steerings: crate::into_domain(result.random_steerings),
        region_pools: crate::into_domain(result.region_pools),
        rules: crate::into_domain(result.rules),
        session_affinity: crate::into_domain(result.session_affinity),
        session_affinity_attributes: crate::into_domain(result.session_affinity_attributes),
        session_affinity_ttl: crate::into_domain(result.session_affinity_ttl),
        steering_policy: crate::into_domain(result.steering_policy),
        ttl: crate::into_domain(result.ttl),
        zone_id: crate::into_domain(result.zone_id),
    }
}
