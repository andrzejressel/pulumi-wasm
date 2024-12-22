use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::zero_trust_tunnel_cloudflared_config;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl zero_trust_tunnel_cloudflared_config::Guest for Component {
    fn invoke(
        name: String,
        args: zero_trust_tunnel_cloudflared_config::Args
    ) -> zero_trust_tunnel_cloudflared_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustTunnelCloudflaredConfig:ZeroTrustTunnelCloudflaredConfig".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "config".into(), value: args.config },
                ObjectField { name: "tunnelId".into(), value: args.tunnel_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "config".into() },
                ResultField { name: "tunnelId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zero_trust_tunnel_cloudflared_config::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            tunnel_id: hashmap.remove("tunnelId").unwrap(),
        }

    }
}
