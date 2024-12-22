use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::load_balancer;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl load_balancer::Guest for Component {
    fn invoke(
        name: String,
        args: load_balancer::Args
    ) -> load_balancer::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancer:LoadBalancer".into(),
            name,
            object: vec![
                ObjectField { name: "adaptiveRoutings".into(), value: args.adaptive_routings },
                ObjectField { name: "countryPools".into(), value: args.country_pools },
                ObjectField { name: "defaultPoolIds".into(), value: args.default_pool_ids },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "fallbackPoolId".into(), value: args.fallback_pool_id },
                ObjectField { name: "locationStrategies".into(), value: args.location_strategies },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "popPools".into(), value: args.pop_pools },
                ObjectField { name: "proxied".into(), value: args.proxied },
                ObjectField { name: "randomSteerings".into(), value: args.random_steerings },
                ObjectField { name: "regionPools".into(), value: args.region_pools },
                ObjectField { name: "rules".into(), value: args.rules },
                ObjectField { name: "sessionAffinity".into(), value: args.session_affinity },
                ObjectField { name: "sessionAffinityAttributes".into(), value: args.session_affinity_attributes },
                ObjectField { name: "sessionAffinityTtl".into(), value: args.session_affinity_ttl },
                ObjectField { name: "steeringPolicy".into(), value: args.steering_policy },
                ObjectField { name: "ttl".into(), value: args.ttl },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "adaptiveRoutings".into() },
                ResultField { name: "countryPools".into() },
                ResultField { name: "createdOn".into() },
                ResultField { name: "defaultPoolIds".into() },
                ResultField { name: "description".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "fallbackPoolId".into() },
                ResultField { name: "locationStrategies".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "name".into() },
                ResultField { name: "popPools".into() },
                ResultField { name: "proxied".into() },
                ResultField { name: "randomSteerings".into() },
                ResultField { name: "regionPools".into() },
                ResultField { name: "rules".into() },
                ResultField { name: "sessionAffinity".into() },
                ResultField { name: "sessionAffinityAttributes".into() },
                ResultField { name: "sessionAffinityTtl".into() },
                ResultField { name: "steeringPolicy".into() },
                ResultField { name: "ttl".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        load_balancer::Res {
            adaptive_routings: hashmap.remove("adaptiveRoutings").unwrap(),
            country_pools: hashmap.remove("countryPools").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            default_pool_ids: hashmap.remove("defaultPoolIds").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            fallback_pool_id: hashmap.remove("fallbackPoolId").unwrap(),
            location_strategies: hashmap.remove("locationStrategies").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            pop_pools: hashmap.remove("popPools").unwrap(),
            proxied: hashmap.remove("proxied").unwrap(),
            random_steerings: hashmap.remove("randomSteerings").unwrap(),
            region_pools: hashmap.remove("regionPools").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
            session_affinity: hashmap.remove("sessionAffinity").unwrap(),
            session_affinity_attributes: hashmap.remove("sessionAffinityAttributes").unwrap(),
            session_affinity_ttl: hashmap.remove("sessionAffinityTtl").unwrap(),
            steering_policy: hashmap.remove("steeringPolicy").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
