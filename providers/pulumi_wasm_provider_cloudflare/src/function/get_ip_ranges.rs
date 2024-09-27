use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_ip_ranges;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_ip_ranges::Guest for Component {
    fn invoke(
    ) -> get_ip_ranges::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getIpRanges:getIpRanges".into(),
            object: vec![
            ],
            results: vec![
                ResultField { name: "chinaIpv4CidrBlocks".into() },
                ResultField { name: "chinaIpv6CidrBlocks".into() },
                ResultField { name: "cidrBlocks".into() },
                ResultField { name: "id".into() },
                ResultField { name: "ipv4CidrBlocks".into() },
                ResultField { name: "ipv6CidrBlocks".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_ip_ranges::Res {
            china_ipv4_cidr_blocks: hashmap.remove("chinaIpv4CidrBlocks").unwrap(),
            china_ipv6_cidr_blocks: hashmap.remove("chinaIpv6CidrBlocks").unwrap(),
            cidr_blocks: hashmap.remove("cidrBlocks").unwrap(),
            id: hashmap.remove("id").unwrap(),
            ipv4_cidr_blocks: hashmap.remove("ipv4CidrBlocks").unwrap(),
            ipv6_cidr_blocks: hashmap.remove("ipv6CidrBlocks").unwrap(),
        }

    }
}
