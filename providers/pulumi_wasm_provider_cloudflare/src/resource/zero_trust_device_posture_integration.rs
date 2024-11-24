use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_device_posture_integration;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_device_posture_integration::Guest for Component {
    fn invoke(name: String, args: zero_trust_device_posture_integration::Args) -> zero_trust_device_posture_integration::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDevicePostureIntegration:ZeroTrustDevicePostureIntegration".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "configs".into(), value: args.configs },
                ObjectField { name: "identifier".into(), value: args.identifier },
                ObjectField { name: "interval".into(), value: args.interval },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "configs".into() },
                ResultField { name: "identifier".into() },
                ResultField { name: "interval".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_device_posture_integration::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configs: hashmap.remove("configs").unwrap(),
            identifier: hashmap.remove("identifier").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
