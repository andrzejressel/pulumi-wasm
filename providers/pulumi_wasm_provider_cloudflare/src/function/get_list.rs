use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_list;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_list::Guest for Component {
    fn invoke(
        name: String,
        args: get_list::Args
    ) -> get_list::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getList:getList".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "description".into() },
                ResultField { name: "id".into() },
                ResultField { name: "kind".into() },
                ResultField { name: "name".into() },
                ResultField { name: "numitems".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_list::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            id: hashmap.remove("id").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
            numitems: hashmap.remove("numitems").unwrap(),
        }

    }
}
