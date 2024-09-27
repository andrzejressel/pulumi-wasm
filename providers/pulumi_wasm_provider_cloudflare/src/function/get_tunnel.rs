use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_tunnel;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_tunnel::Guest for Component {
    fn invoke(
        args: get_tunnel::Args
    ) -> get_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getTunnel:getTunnel".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "name".into(), value: args.name },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "id".into() },
                ResultField { name: "name".into() },
                ResultField { name: "remoteConfig".into() },
                ResultField { name: "status".into() },
                ResultField { name: "tunnelType".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            id: hashmap.remove("id").unwrap(),
            name: hashmap.remove("name").unwrap(),
            remote_config: hashmap.remove("remoteConfig").unwrap(),
            status: hashmap.remove("status").unwrap(),
            tunnel_type: hashmap.remove("tunnelType").unwrap(),
        }

    }
}
