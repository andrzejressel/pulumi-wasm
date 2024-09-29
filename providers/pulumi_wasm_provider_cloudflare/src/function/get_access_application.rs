use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_access_application;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_access_application::Guest for Component {
    fn invoke(
        args: get_access_application::Args
    ) -> get_access_application::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getAccessApplication:getAccessApplication".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "domain".into(), value: args.domain },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "aud".into() },
                ResultField { name: "domain".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_access_application::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            aud: hashmap.remove("aud").unwrap(),
            domain: hashmap.remove("domain").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
