use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_load_balancer_pools;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_load_balancer_pools::Guest for Component {
    fn invoke(
        args: get_load_balancer_pools::Args
    ) -> get_load_balancer_pools::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getLoadBalancerPools:getLoadBalancerPools".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "filter".into(), value: args.filter },
                ObjectField { name: "pools".into(), value: args.pools },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "filter".into() },
                ResultField { name: "id".into() },
                ResultField { name: "pools".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_load_balancer_pools::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            filter: hashmap.remove("filter").unwrap(),
            id: hashmap.remove("id").unwrap(),
            pools: hashmap.remove("pools").unwrap(),
        }
    }
}
