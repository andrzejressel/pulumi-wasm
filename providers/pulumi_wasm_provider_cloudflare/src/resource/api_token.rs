use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::api_token;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl api_token::Guest for Component {
    fn invoke(
        name: String,
        args: api_token::Args
    ) -> api_token::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiToken:ApiToken".into(),
            name,
            object: vec![
                ObjectField { name: "condition".into(), value: args.condition },
                ObjectField { name: "expiresOn".into(), value: args.expires_on },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "notBefore".into(), value: args.not_before },
                ObjectField { name: "policies".into(), value: args.policies },
            ],
            results: vec![
                ResultField { name: "condition".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "issuedOn".into() },
                ResultField { name: "modifiedOn".into() },
                ResultField { name: "name".into() },
                ResultField { name: "notBefore".into() },
                ResultField { name: "policies".into() },
                ResultField { name: "status".into() },
                ResultField { name: "value".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        api_token::Res {
            condition: hashmap.remove("condition").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issued_on: hashmap.remove("issuedOn").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            not_before: hashmap.remove("notBefore").unwrap(),
            policies: hashmap.remove("policies").unwrap(),
            status: hashmap.remove("status").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }
    }
}
