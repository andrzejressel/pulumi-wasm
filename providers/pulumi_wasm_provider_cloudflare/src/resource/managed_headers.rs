use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::managed_headers;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl managed_headers::Guest for Component {
    fn invoke(
        name: String,
        args: managed_headers::Args
    ) -> managed_headers::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/managedHeaders:ManagedHeaders".into(),
            name,
            object: vec![
                ObjectField { name: "managedRequestHeaders".into(), value: args.managed_request_headers },
                ObjectField { name: "managedResponseHeaders".into(), value: args.managed_response_headers },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "managedRequestHeaders".into() },
                ResultField { name: "managedResponseHeaders".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        managed_headers::Res {
            managed_request_headers: hashmap.remove("managedRequestHeaders").unwrap(),
            managed_response_headers: hashmap.remove("managedResponseHeaders").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
