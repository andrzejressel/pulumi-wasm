use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_gateway_proxy_endpoint;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_gateway_proxy_endpoint::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_gateway_proxy_endpoint::Args
    ) -> zero_trust_gateway_proxy_endpoint::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayProxyEndpoint:ZeroTrustGatewayProxyEndpoint".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "ips".into(), value: args.ips },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "ips".into() },
                ResultField { name: "name".into() },
                ResultField { name: "subdomain".into() },
            ],
        };

        let o = register(&request);
        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();
        zero_trust_gateway_proxy_endpoint::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            ips: hashmap.remove("ips").unwrap(),
            name: hashmap.remove("name").unwrap(),
            subdomain: hashmap.remove("subdomain").unwrap(),
        }
    }
}
