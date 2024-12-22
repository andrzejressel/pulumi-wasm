use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::argo;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl argo::Guest for Component {
    fn invoke(
        name: String,
        args: argo::Args
    ) -> argo::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/argo:Argo".into(),
            name,
            object: vec![
                ObjectField { name: "smartRouting".into(), value: args.smart_routing },
                ObjectField { name: "tieredCaching".into(), value: args.tiered_caching },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "smartRouting".into() },
                ResultField { name: "tieredCaching".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        argo::Res {
            smart_routing: hashmap.remove("smartRouting").unwrap(),
            tiered_caching: hashmap.remove("tieredCaching").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
