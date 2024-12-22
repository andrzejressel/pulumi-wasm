use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::access_identity_provider;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl access_identity_provider::Guest for Component {
    fn invoke(
        name: String,
        args: access_identity_provider::Args
    ) -> access_identity_provider::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessIdentityProvider:AccessIdentityProvider".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "configs".into(), value: args.configs },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "scimConfigs".into(), value: args.scim_configs },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "configs".into() },
                ResultField { name: "name".into() },
                ResultField { name: "scimConfigs".into() },
                ResultField { name: "type".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        access_identity_provider::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configs: hashmap.remove("configs").unwrap(),
            name: hashmap.remove("name").unwrap(),
            scim_configs: hashmap.remove("scimConfigs").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
