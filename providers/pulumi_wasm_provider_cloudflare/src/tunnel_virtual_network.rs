use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::tunnel_virtual_network;
use crate::Component;
use std::collections::HashMap;

impl tunnel_virtual_network::Guest for Component {
    fn invoke(name: String, args: tunnel_virtual_network::Args) -> tunnel_virtual_network::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "comment".into(),
                    value: args.comment,
                },
                ObjectField {
                    name: "isDefaultNetwork".into(),
                    value: args.is_default_network,
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
                    name: "comment".into(),
                },
                ResultField {
                    name: "isDefaultNetwork".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_virtual_network::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            is_default_network: hashmap.remove("isDefaultNetwork").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
