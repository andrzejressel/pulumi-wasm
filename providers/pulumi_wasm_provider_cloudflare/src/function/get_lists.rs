use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_lists;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_lists::Guest for Component {
    fn invoke(
        args: get_lists::Args
    ) -> get_lists::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getLists:getLists".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "lists".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_lists::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            lists: hashmap.remove("lists").unwrap(),
        }
    }
}
