use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::device_policy_certificates;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl device_policy_certificates::Guest for Component {
    fn invoke(
        name: String,
        args: device_policy_certificates::Args
    ) -> device_policy_certificates::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/devicePolicyCertificates:DevicePolicyCertificates".into(),
            name,
            object: vec![
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "enabled".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        device_policy_certificates::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
