use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_api_token_permission_groups;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_api_token_permission_groups::Guest for Component {
    fn invoke(
        name: String,
    ) -> get_api_token_permission_groups::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getApiTokenPermissionGroups:getApiTokenPermissionGroups".into(),
            object: vec![
            ],
            results: vec![
                ResultField { name: "account".into() },
                ResultField { name: "id".into() },
                ResultField { name: "permissions".into() },
                ResultField { name: "r2".into() },
                ResultField { name: "user".into() },
                ResultField { name: "zone".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_api_token_permission_groups::Res {
            account: hashmap.remove("account").unwrap(),
            id: hashmap.remove("id").unwrap(),
            permissions: hashmap.remove("permissions").unwrap(),
            r2: hashmap.remove("r2").unwrap(),
            user: hashmap.remove("user").unwrap(),
            zone: hashmap.remove("zone").unwrap(),
        }

    }
}
