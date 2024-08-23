use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::web3_hostname;
use crate::Component;
use std::collections::HashMap;

impl web3_hostname::Guest for Component {
    fn invoke(name: String, args: web3_hostname::Args) -> web3_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/web3Hostname:Web3Hostname".into(),
            name,
            object: vec![
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "dnslink".into(),
                    value: args.dnslink,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "target".into(),
                    value: args.target,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "dnslink".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "target".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        web3_hostname::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            dnslink: hashmap.remove("dnslink").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            status: hashmap.remove("status").unwrap(),
            target: hashmap.remove("target").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
