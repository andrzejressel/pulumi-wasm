use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::gre_tunnel;
use crate::Component;
use std::collections::HashMap;

impl gre_tunnel::Guest for Component {
    fn invoke(name: String, args: gre_tunnel::Args) -> gre_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/greTunnel:GreTunnel".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "cloudflareGreEndpoint".into(),
                    value: args.cloudflare_gre_endpoint,
                },
                ObjectField {
                    name: "customerGreEndpoint".into(),
                    value: args.customer_gre_endpoint,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "healthCheckEnabled".into(),
                    value: args.health_check_enabled,
                },
                ObjectField {
                    name: "healthCheckTarget".into(),
                    value: args.health_check_target,
                },
                ObjectField {
                    name: "healthCheckType".into(),
                    value: args.health_check_type,
                },
                ObjectField {
                    name: "interfaceAddress".into(),
                    value: args.interface_address,
                },
                ObjectField {
                    name: "mtu".into(),
                    value: args.mtu,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "ttl".into(),
                    value: args.ttl,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "cloudflareGreEndpoint".into(),
                },
                ResultField {
                    name: "customerGreEndpoint".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "healthCheckEnabled".into(),
                },
                ResultField {
                    name: "healthCheckTarget".into(),
                },
                ResultField {
                    name: "healthCheckType".into(),
                },
                ResultField {
                    name: "interfaceAddress".into(),
                },
                ResultField { name: "mtu".into() },
                ResultField {
                    name: "name".into(),
                },
                ResultField { name: "ttl".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        gre_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            cloudflare_gre_endpoint: hashmap.remove("cloudflareGreEndpoint").unwrap(),
            customer_gre_endpoint: hashmap.remove("customerGreEndpoint").unwrap(),
            description: hashmap.remove("description").unwrap(),
            health_check_enabled: hashmap.remove("healthCheckEnabled").unwrap(),
            health_check_target: hashmap.remove("healthCheckTarget").unwrap(),
            health_check_type: hashmap.remove("healthCheckType").unwrap(),
            interface_address: hashmap.remove("interfaceAddress").unwrap(),
            mtu: hashmap.remove("mtu").unwrap(),
            name: hashmap.remove("name").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
        }
    }
}