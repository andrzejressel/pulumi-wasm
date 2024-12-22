use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zero_trust_tunnel_virtual_network;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zero_trust_tunnel_virtual_network::Guest for Component {
    fn invoke(
        args: get_zero_trust_tunnel_virtual_network::Args
    ) -> get_zero_trust_tunnel_virtual_network::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZeroTrustTunnelVirtualNetwork:getZeroTrustTunnelVirtualNetwork".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "comment".into() },
                ResultField { name: "id".into() },
                ResultField { name: "isDefault".into() },
                ResultField { name: "name".into() },
            ],
        };

        let o = invoke(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        get_zero_trust_tunnel_virtual_network::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            id: hashmap.remove("id").unwrap(),
            is_default: hashmap.remove("isDefault").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
