use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::tunnel_config;
use crate::Component;
use std::collections::HashMap;

impl tunnel_config::Guest for Component {
    fn invoke(name: String, args: tunnel_config::Args) -> tunnel_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelConfig:TunnelConfig".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "config".into(),
                    value: args.config,
                },
                ObjectField {
                    name: "tunnelId".into(),
                    value: args.tunnel_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "config".into(),
                },
                ResultField {
                    name: "tunnelId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_config::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            tunnel_id: hashmap.remove("tunnelId").unwrap(),
        }
    }
}
