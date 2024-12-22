use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_user;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_user::Guest for Component {
    fn invoke(
    ) -> get_user::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getUser:getUser".into(),
            object: vec![
            ],
            results: vec![
                ResultField { name: "email".into() },
                ResultField { name: "id".into() },
                ResultField { name: "username".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_user::Res {
            email: hashmap.remove("email").unwrap(),
            id: hashmap.remove("id").unwrap(),
            username: hashmap.remove("username").unwrap(),
        }
    }
}
