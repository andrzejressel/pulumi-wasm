use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::teams_list;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl teams_list::Guest for Component {
    fn invoke(name: String, args: teams_list::Args) -> teams_list::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsList:TeamsList".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "description".into(), value: args.description },
                ObjectField { name: "items".into(), value: args.items },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "type".into(), value: args.type_ },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "description".into() },
                ResultField { name: "items".into() },
                ResultField { name: "name".into() },
                ResultField { name: "type".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_list::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            items: hashmap.remove("items").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }

    }
}
