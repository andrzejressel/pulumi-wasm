use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::regional_hostname;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl regional_hostname::Guest for Component {
    fn invoke(
        name: String,
        args: regional_hostname::Args
    ) -> regional_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/regionalHostname:RegionalHostname".into(),
            name,
            object: vec![
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "regionKey".into(), value: args.region_key },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "createdOn".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "regionKey".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        regional_hostname::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            region_key: hashmap.remove("regionKey").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
