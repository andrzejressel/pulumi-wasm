use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_accounts;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_accounts::Guest for Component {
    fn invoke(
        args: get_accounts::Args
    ) -> get_accounts::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getAccounts:getAccounts".into(),
            object: vec![
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accounts".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_accounts::Res {
            accounts: hashmap.remove("accounts").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
