pub struct LoadBalancerArgs {
    pub adaptive_routings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    pub location_strategies:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    pub random_steerings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    pub session_affinity_attributes:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    pub steering_policy: pulumi_wasm_rust::Output<Option<String>>,
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct LoadBalancerResult {
    pub adaptive_routings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    pub created_on: pulumi_wasm_rust::Output<String>,
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    pub location_strategies:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    pub random_steerings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    pub session_affinity_attributes:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    pub steering_policy: pulumi_wasm_rust::Output<String>,
    pub ttl: pulumi_wasm_rust::Output<i32>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: LoadBalancerArgs) -> LoadBalancerResult {
    let result = crate::bindings::pulumi::cloudflare::load_balancer::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::load_balancer::Args {
            adaptive_routings: args.adaptive_routings.get_inner(),
            country_pools: args.country_pools.get_inner(),
            default_pool_ids: args.default_pool_ids.get_inner(),
            description: args.description.get_inner(),
            enabled: args.enabled.get_inner(),
            fallback_pool_id: args.fallback_pool_id.get_inner(),
            location_strategies: args.location_strategies.get_inner(),
            name: args.name.get_inner(),
            pop_pools: args.pop_pools.get_inner(),
            proxied: args.proxied.get_inner(),
            random_steerings: args.random_steerings.get_inner(),
            region_pools: args.region_pools.get_inner(),
            rules: args.rules.get_inner(),
            session_affinity: args.session_affinity.get_inner(),
            session_affinity_attributes: args.session_affinity_attributes.get_inner(),
            session_affinity_ttl: args.session_affinity_ttl.get_inner(),
            steering_policy: args.steering_policy.get_inner(),
            ttl: args.ttl.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

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
