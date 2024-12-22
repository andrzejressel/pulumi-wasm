use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_dcv_delegation;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_dcv_delegation::Guest for Component {
    fn invoke(
        args: get_dcv_delegation::Args
    ) -> get_dcv_delegation::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getDcvDelegation:getDcvDelegation".into(),
            object: vec![
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "hostname".into() },
                ResultField { name: "id".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_dcv_delegation::Res {
            hostname: hashmap.remove("hostname").unwrap(),
            id: hashmap.remove("id").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
