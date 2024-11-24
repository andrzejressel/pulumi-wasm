use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_device_managed_networks;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_device_managed_networks::Guest for Component {
    fn invoke(name: String, args: zero_trust_device_managed_networks::Args) -> zero_trust_device_managed_networks::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDeviceManagedNetworks:ZeroTrustDeviceManagedNetworks".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "config".into(), value: args.config },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "config".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_device_managed_networks::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
