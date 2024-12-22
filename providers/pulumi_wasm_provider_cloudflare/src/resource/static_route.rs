use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::static_route;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl static_route::Guest for Component {
    fn invoke(
        name: String,
        args: static_route::Args
    ) -> static_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/staticRoute:StaticRoute".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "coloNames".into(), value: args.colo_names },
                ObjectField { name: "coloRegions".into(), value: args.colo_regions },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "nexthop".into(), value: args.nexthop },
                ObjectField { name: "prefix".into(), value: args.prefix },
                ObjectField { name: "priority".into(), value: args.priority },
                ObjectField { name: "weight".into(), value: args.weight },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "coloNames".into() },
                ResultField { name: "coloRegions".into() },
                ResultField { name: "description".into() },
                ResultField { name: "nexthop".into() },
                ResultField { name: "prefix".into() },
                ResultField { name: "priority".into() },
                ResultField { name: "weight".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        static_route::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            colo_names: hashmap.remove("coloNames").unwrap(),
            colo_regions: hashmap.remove("coloRegions").unwrap(),
            description: hashmap.remove("description").unwrap(),
            nexthop: hashmap.remove("nexthop").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            weight: hashmap.remove("weight").unwrap(),
        }

    }
}
