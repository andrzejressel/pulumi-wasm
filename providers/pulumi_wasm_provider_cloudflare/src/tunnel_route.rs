use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::tunnel_route;
use crate::Component;
use std::collections::HashMap;

impl tunnel_route::Guest for Component {
    fn invoke(name: String, args: tunnel_route::Args) -> tunnel_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelRoute:TunnelRoute".into(),
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
                    name: "network".into(),
                    value: args.network,
                },
                ObjectField {
                    name: "tunnelId".into(),
                    value: args.tunnel_id,
                },
                ObjectField {
                    name: "virtualNetworkId".into(),
                    value: args.virtual_network_id,
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
                    name: "network".into(),
                },
                ResultField {
                    name: "tunnelId".into(),
                },
                ResultField {
                    name: "virtualNetworkId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_route::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            network: hashmap.remove("network").unwrap(),
            tunnel_id: hashmap.remove("tunnelId").unwrap(),
            virtual_network_id: hashmap.remove("virtualNetworkId").unwrap(),
        }
    }
}
