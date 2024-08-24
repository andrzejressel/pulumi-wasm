use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::spectrum_application;
use crate::Component;
use std::collections::HashMap;

impl spectrum_application::Guest for Component {
    fn invoke(name: String, args: spectrum_application::Args) -> spectrum_application::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/spectrumApplication:SpectrumApplication".into(),
            name,
            object: vec![
                ObjectField {
                    name: "argoSmartRouting".into(),
                    value: args.argo_smart_routing,
                },
                ObjectField {
                    name: "dns".into(),
                    value: args.dns,
                },
                ObjectField {
                    name: "edgeIps".into(),
                    value: args.edge_ips,
                },
                ObjectField {
                    name: "ipFirewall".into(),
                    value: args.ip_firewall,
                },
                ObjectField {
                    name: "originDirects".into(),
                    value: args.origin_directs,
                },
                ObjectField {
                    name: "originDns".into(),
                    value: args.origin_dns,
                },
                ObjectField {
                    name: "originPort".into(),
                    value: args.origin_port,
                },
                ObjectField {
                    name: "originPortRange".into(),
                    value: args.origin_port_range,
                },
                ObjectField {
                    name: "protocol".into(),
                    value: args.protocol,
                },
                ObjectField {
                    name: "proxyProtocol".into(),
                    value: args.proxy_protocol,
                },
                ObjectField {
                    name: "tls".into(),
                    value: args.tls,
                },
                ObjectField {
                    name: "trafficType".into(),
                    value: args.traffic_type,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "argoSmartRouting".into(),
                },
                ResultField { name: "dns".into() },
                ResultField {
                    name: "edgeIps".into(),
                },
                ResultField {
                    name: "ipFirewall".into(),
                },
                ResultField {
                    name: "originDirects".into(),
                },
                ResultField {
                    name: "originDns".into(),
                },
                ResultField {
                    name: "originPort".into(),
                },
                ResultField {
                    name: "originPortRange".into(),
                },
                ResultField {
                    name: "protocol".into(),
                },
                ResultField {
                    name: "proxyProtocol".into(),
                },
                ResultField { name: "tls".into() },
                ResultField {
                    name: "trafficType".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        spectrum_application::Res {
            argo_smart_routing: hashmap.remove("argoSmartRouting").unwrap(),
            dns: hashmap.remove("dns").unwrap(),
            edge_ips: hashmap.remove("edgeIps").unwrap(),
            ip_firewall: hashmap.remove("ipFirewall").unwrap(),
            origin_directs: hashmap.remove("originDirects").unwrap(),
            origin_dns: hashmap.remove("originDns").unwrap(),
            origin_port: hashmap.remove("originPort").unwrap(),
            origin_port_range: hashmap.remove("originPortRange").unwrap(),
            protocol: hashmap.remove("protocol").unwrap(),
            proxy_protocol: hashmap.remove("proxyProtocol").unwrap(),
            tls: hashmap.remove("tls").unwrap(),
            traffic_type: hashmap.remove("trafficType").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
