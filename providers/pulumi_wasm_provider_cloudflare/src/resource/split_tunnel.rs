use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::split_tunnel;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl split_tunnel::Guest for Component {
    fn invoke(
        name: String,
        args: split_tunnel::Args
    ) -> split_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/splitTunnel:SplitTunnel".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "mode".into(), value: args.mode },
                ObjectField { name: "policyId".into(), value: args.policy_id },
                ObjectField { name: "tunnels".into(), value: args.tunnels },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "mode".into() },
                ResultField { name: "policyId".into() },
                ResultField { name: "tunnels".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        split_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            policy_id: hashmap.remove("policyId").unwrap(),
            tunnels: hashmap.remove("tunnels").unwrap(),
        }

    }
}
