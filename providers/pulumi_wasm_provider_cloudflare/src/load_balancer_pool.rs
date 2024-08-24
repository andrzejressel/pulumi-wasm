use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::load_balancer_pool;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl load_balancer_pool::Guest for Component {
    fn invoke(name: String, args: load_balancer_pool::Args) -> load_balancer_pool::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerPool:LoadBalancerPool".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "checkRegions".into(), value: args.check_regions },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "latitude".into(), value: args.latitude },
                ObjectField { name: "loadSheddings".into(), value: args.load_sheddings },
                ObjectField { name: "longitude".into(), value: args.longitude },
                ObjectField { name: "minimumOrigins".into(), value: args.minimum_origins },
                ObjectField { name: "monitor".into(), value: args.monitor },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "notificationEmail".into(), value: args.notification_email },
                ObjectField { name: "originSteerings".into(), value: args.origin_steerings },
                ObjectField { name: "origins".into(), value: args.origins },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "checkRegions".into() },
                ResultField { name: "createdOn".into() },
                ResultField { name: "description".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "latitude".into() },
                ResultField { name: "loadSheddings".into() },
                ResultField { name: "longitude".into() },
                ResultField { name: "minimumOrigins".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "monitor".into() },
                ResultField { name: "name".into() },
                ResultField { name: "notificationEmail".into() },
                ResultField { name: "originSteerings".into() },
                ResultField { name: "origins".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        load_balancer_pool::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            check_regions: hashmap.remove("checkRegions").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            latitude: hashmap.remove("latitude").unwrap(),
            load_sheddings: hashmap.remove("loadSheddings").unwrap(),
            longitude: hashmap.remove("longitude").unwrap(),
            minimum_origins: hashmap.remove("minimumOrigins").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            monitor: hashmap.remove("monitor").unwrap(),
            name: hashmap.remove("name").unwrap(),
            notification_email: hashmap.remove("notificationEmail").unwrap(),
            origin_steerings: hashmap.remove("originSteerings").unwrap(),
            origins: hashmap.remove("origins").unwrap(),
        }

    }
}
