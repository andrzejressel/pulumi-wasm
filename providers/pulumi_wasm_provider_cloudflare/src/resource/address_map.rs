use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::address_map;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl address_map::Guest for Component {
    fn invoke(name: String, args: address_map::Args) -> address_map::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/addressMap:AddressMap".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "defaultSni".into(), value: args.default_sni },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "enabled".into(), value: args.enabled },
                ObjectField { name: "ips".into(), value: args.ips },
                ObjectField { name: "memberships".into(), value: args.memberships },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "canDelete".into() },
                ResultField { name: "canModifyIps".into() },
                ResultField { name: "defaultSni".into() },
                ResultField { name: "description".into() },
                ResultField { name: "enabled".into() },
                ResultField { name: "ips".into() },
                ResultField { name: "memberships".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        address_map::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            can_delete: hashmap.remove("canDelete").unwrap(),
            can_modify_ips: hashmap.remove("canModifyIps").unwrap(),
            default_sni: hashmap.remove("defaultSni").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            ips: hashmap.remove("ips").unwrap(),
            memberships: hashmap.remove("memberships").unwrap(),
        }

    }
}
