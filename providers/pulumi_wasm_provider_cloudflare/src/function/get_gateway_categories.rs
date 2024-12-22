use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_gateway_categories;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_gateway_categories::Guest for Component {
    fn invoke(
        args: get_gateway_categories::Args
    ) -> get_gateway_categories::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getGatewayCategories:getGatewayCategories".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "categories".into() },
                ResultField { name: "id".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_gateway_categories::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            categories: hashmap.remove("categories").unwrap(),
            id: hashmap.remove("id").unwrap(),
        }
    }
}
