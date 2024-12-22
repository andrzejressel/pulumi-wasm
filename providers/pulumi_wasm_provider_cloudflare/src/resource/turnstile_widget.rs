use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::turnstile_widget;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl turnstile_widget::Guest for Component {
    fn invoke(
        name: String,
        args: turnstile_widget::Args
    ) -> turnstile_widget::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/turnstileWidget:TurnstileWidget".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "botFightMode".into(), value: args.bot_fight_mode },
                ObjectField { name: "domains".into(), value: args.domains },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "offlabel".into(), value: args.offlabel },
                ObjectField { name: "region".into(), value: args.region },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "botFightMode".into() },
                ResultField { name: "domains".into() },
                ResultField { name: "mode".into() },
                ResultField { name: "name".into() },
                ResultField { name: "offlabel".into() },
                ResultField { name: "region".into() },
                ResultField { name: "secret".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        turnstile_widget::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            bot_fight_mode: hashmap.remove("botFightMode").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            name: hashmap.remove("name").unwrap(),
            offlabel: hashmap.remove("offlabel").unwrap(),
            region: hashmap.remove("region").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
        }
    }
}
