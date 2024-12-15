use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zero_trust_tunnel_cloudflared;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zero_trust_tunnel_cloudflared::Guest for Component {
    fn invoke(
        args: get_zero_trust_tunnel_cloudflared::Args
    ) -> get_zero_trust_tunnel_cloudflared::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZeroTrustTunnelCloudflared:getZeroTrustTunnelCloudflared".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "isDeleted".into(), value: args.is_deleted },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "isDeleted".into() },
                ResultField { name: "name".into() },
                ResultField { name: "remoteConfig".into() },
                ResultField { name: "status".into() },
                ResultField { name: "tunnelType".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zero_trust_tunnel_cloudflared::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            is_deleted: hashmap.remove("isDeleted").unwrap(),
            name: hashmap.remove("name").unwrap(),
            remote_config: hashmap.remove("remoteConfig").unwrap(),
            status: hashmap.remove("status").unwrap(),
            tunnel_type: hashmap.remove("tunnelType").unwrap(),
        }
    }
}
