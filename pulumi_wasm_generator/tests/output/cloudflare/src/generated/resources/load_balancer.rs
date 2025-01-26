/// Provides a Cloudflare Load Balancer resource. This sits in front of
/// a number of defined pools of origins and provides various options
/// for geographically-aware load balancing. Note that the load balancing
/// feature must be enabled in your Cloudflare account before you can use
/// this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = load_balancer::create(
///         "example",
///         LoadBalancerArgs::builder()
///             .country_pools(
///                 vec![
///                     LoadBalancerCountryPool::builder().country("US")
///                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).build_struct(),
///                 ],
///             )
///             .default_pool_ids(vec!["${exampleLoadBalancerPool.id}",])
///             .description("example load balancer using geo-balancing")
///             .fallback_pool_id("${exampleLoadBalancerPool.id}")
///             .name("example-load-balancer.example.com")
///             .pop_pools(
///                 vec![
///                     LoadBalancerPopPool::builder()
///                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).pop("LAX")
///                     .build_struct(),
///                 ],
///             )
///             .proxied(true)
///             .region_pools(
///                 vec![
///                     LoadBalancerRegionPool::builder()
///                     .poolIds(vec!["${exampleLoadBalancerPool.id}",]).region("WNAM")
///                     .build_struct(),
///                 ],
///             )
///             .rules(
///                 vec![
///                     LoadBalancerRule::builder()
///                     .condition("http.request.uri.path contains \"testing\"")
///                     .fixedResponse(LoadBalancerRuleFixedResponse::builder()
///                     .contentType("html").location("www.example.com").messageBody("hello")
///                     .statusCode(200).build_struct()).name("example rule").build_struct(),
///                 ],
///             )
///             .steering_policy("geo")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let exampleLoadBalancerPool = load_balancer_pool::create(
///         "exampleLoadBalancerPool",
///         LoadBalancerPoolArgs::builder()
///             .name("example-lb-pool")
///             .origins(
///                 vec![
///                     LoadBalancerPoolOrigin::builder().address("192.0.2.1").enabled(false)
///                     .name("example-1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/loadBalancer:LoadBalancer example <zone_id>/<load_balancer_id>
/// ```
///
pub mod load_balancer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerArgs {
        /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
        #[builder(into, default)]
        pub adaptive_routings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerAdaptiveRouting>>,
        >,
        /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
        #[builder(into, default)]
        pub country_pools: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerCountryPool>>,
        >,
        /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
        #[builder(into)]
        pub default_pool_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Free text description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enable or disable the load balancer. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The pool ID to use when all other pools are detected as unhealthy.
        #[builder(into)]
        pub fallback_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Controls location-based steering for non-proxied requests.
        #[builder(into, default)]
        pub location_strategies: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerLocationStrategy>>,
        >,
        /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
        #[builder(into, default)]
        pub pop_pools: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerPopPool>>,
        >,
        /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
        #[builder(into, default)]
        pub proxied: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
        #[builder(into, default)]
        pub random_steerings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerRandomSteering>>,
        >,
        /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
        #[builder(into, default)]
        pub region_pools: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerRegionPool>>,
        >,
        /// A list of rules for this load balancer to execute.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerRule>>,
        >,
        /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
        #[builder(into, default)]
        pub session_affinity: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configure attributes for session affinity.
        #[builder(into, default)]
        pub session_affinity_attributes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerSessionAffinityAttribute>>,
        >,
        /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
        #[builder(into, default)]
        pub session_affinity_ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
        #[builder(into, default)]
        pub steering_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerResult {
        /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
        pub adaptive_routings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerAdaptiveRouting>>,
        >,
        /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
        pub country_pools: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerCountryPool>>,
        >,
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
        pub location_strategies: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerLocationStrategy>>,
        >,
        /// The RFC3339 timestamp of when the load balancer was last modified.
        pub modified_on: pulumi_wasm_rust::Output<String>,
        /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
        pub pop_pools: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerPopPool>>,
        >,
        /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
        pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
        pub random_steerings: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerRandomSteering>>,
        >,
        /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
        pub region_pools: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerRegionPool>>,
        >,
        /// A list of rules for this load balancer to execute.
        pub rules: pulumi_wasm_rust::Output<Option<Vec<super::types::LoadBalancerRule>>>,
        /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
        pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
        /// Configure attributes for session affinity.
        pub session_affinity_attributes: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerSessionAffinityAttribute>>,
        >,
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoadBalancerArgs,
    ) -> LoadBalancerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let adaptive_routings_binding = args
            .adaptive_routings
            .get_output(context)
            .get_inner();
        let country_pools_binding = args.country_pools.get_output(context).get_inner();
        let default_pool_ids_binding = args
            .default_pool_ids
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let fallback_pool_id_binding = args
            .fallback_pool_id
            .get_output(context)
            .get_inner();
        let location_strategies_binding = args
            .location_strategies
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pop_pools_binding = args.pop_pools.get_output(context).get_inner();
        let proxied_binding = args.proxied.get_output(context).get_inner();
        let random_steerings_binding = args
            .random_steerings
            .get_output(context)
            .get_inner();
        let region_pools_binding = args.region_pools.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let session_affinity_binding = args
            .session_affinity
            .get_output(context)
            .get_inner();
        let session_affinity_attributes_binding = args
            .session_affinity_attributes
            .get_output(context)
            .get_inner();
        let session_affinity_ttl_binding = args
            .session_affinity_ttl
            .get_output(context)
            .get_inner();
        let steering_policy_binding = args
            .steering_policy
            .get_output(context)
            .get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancer:LoadBalancer".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adaptiveRoutings".into(),
                    value: &adaptive_routings_binding,
                },
                register_interface::ObjectField {
                    name: "countryPools".into(),
                    value: &country_pools_binding,
                },
                register_interface::ObjectField {
                    name: "defaultPoolIds".into(),
                    value: &default_pool_ids_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "fallbackPoolId".into(),
                    value: &fallback_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "locationStrategies".into(),
                    value: &location_strategies_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "popPools".into(),
                    value: &pop_pools_binding,
                },
                register_interface::ObjectField {
                    name: "proxied".into(),
                    value: &proxied_binding,
                },
                register_interface::ObjectField {
                    name: "randomSteerings".into(),
                    value: &random_steerings_binding,
                },
                register_interface::ObjectField {
                    name: "regionPools".into(),
                    value: &region_pools_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinity".into(),
                    value: &session_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinityAttributes".into(),
                    value: &session_affinity_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinityTtl".into(),
                    value: &session_affinity_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "steeringPolicy".into(),
                    value: &steering_policy_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adaptiveRoutings".into(),
                },
                register_interface::ResultField {
                    name: "countryPools".into(),
                },
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "defaultPoolIds".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "fallbackPoolId".into(),
                },
                register_interface::ResultField {
                    name: "locationStrategies".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "popPools".into(),
                },
                register_interface::ResultField {
                    name: "proxied".into(),
                },
                register_interface::ResultField {
                    name: "randomSteerings".into(),
                },
                register_interface::ResultField {
                    name: "regionPools".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinity".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinityAttributes".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinityTtl".into(),
                },
                register_interface::ResultField {
                    name: "steeringPolicy".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoadBalancerResult {
            adaptive_routings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adaptiveRoutings").unwrap(),
            ),
            country_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("countryPools").unwrap(),
            ),
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            default_pool_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPoolIds").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            fallback_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fallbackPoolId").unwrap(),
            ),
            location_strategies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationStrategies").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pop_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("popPools").unwrap(),
            ),
            proxied: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxied").unwrap(),
            ),
            random_steerings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("randomSteerings").unwrap(),
            ),
            region_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionPools").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            session_affinity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinity").unwrap(),
            ),
            session_affinity_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinityAttributes").unwrap(),
            ),
            session_affinity_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinityTtl").unwrap(),
            ),
            steering_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("steeringPolicy").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
