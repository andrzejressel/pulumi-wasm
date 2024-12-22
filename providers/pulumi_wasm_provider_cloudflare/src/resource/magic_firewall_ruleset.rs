use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::magic_firewall_ruleset;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl magic_firewall_ruleset::Guest for Component {
    fn invoke(
        name: String,
        args: magic_firewall_ruleset::Args
    ) -> magic_firewall_ruleset::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "rules".into(), value: args.rules },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "description".into() },
                ResultField { name: "name".into() },
                ResultField { name: "rules".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        magic_firewall_ruleset::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            name: hashmap.remove("name").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
        }
    }
}
