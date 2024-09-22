use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::list_item;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl list_item::Guest for Component {
    fn invoke(name: String, args: list_item::Args) -> list_item::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/listItem:ListItem".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "asn".into(), value: args.asn },
                ObjectField { name: "comment".into(), value: args.comment },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "ip".into(), value: args.ip },
                ObjectField { name: "listId".into(), value: args.list_id },
                ObjectField { name: "redirect".into(), value: args.redirect },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "asn".into() },
                ResultField { name: "comment".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "ip".into() },
                ResultField { name: "listId".into() },
                ResultField { name: "redirect".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list_item::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            asn: hashmap.remove("asn").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ip: hashmap.remove("ip").unwrap(),
            list_id: hashmap.remove("listId").unwrap(),
            redirect: hashmap.remove("redirect").unwrap(),
        }

    }
}
