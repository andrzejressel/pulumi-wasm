use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_tunnel_cloudflared;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_tunnel_cloudflared::Guest for Component {
    fn invoke(name: String, args: zero_trust_tunnel_cloudflared::Args) -> zero_trust_tunnel_cloudflared::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelCloudflared:ZeroTrustTunnelCloudflared".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "configSrc".into(), value: args.config_src },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "secret".into(), value: args.secret },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "cname".into() },
                ResultField { name: "configSrc".into() },
                ResultField { name: "name".into() },
                ResultField { name: "secret".into() },
                ResultField { name: "tunnelToken".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_tunnel_cloudflared::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            cname: hashmap.remove("cname").unwrap(),
            config_src: hashmap.remove("configSrc").unwrap(),
            name: hashmap.remove("name").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
            tunnel_token: hashmap.remove("tunnelToken").unwrap(),
        }

    }
}
