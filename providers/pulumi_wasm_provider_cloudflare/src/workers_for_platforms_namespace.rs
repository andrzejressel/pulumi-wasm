use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::workers_for_platforms_namespace;
use crate::Component;
use std::collections::HashMap;

impl workers_for_platforms_namespace::Guest for Component {
    fn invoke(
        name: String,
        args: workers_for_platforms_namespace::Args,
    ) -> workers_for_platforms_namespace::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace"
                .into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        workers_for_platforms_namespace::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
