use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::get_zero_trust_infrastructure_access_targets;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, invoke, ResourceInvokeRequest, ResultField};
use crate::Component;

impl get_zero_trust_infrastructure_access_targets::Guest for Component {
    fn invoke(
        args: get_zero_trust_infrastructure_access_targets::Args
    ) -> get_zero_trust_infrastructure_access_targets::Res {
        pulumi_wasm_common::setup_logger();
        let request = ResourceInvokeRequest {
            token: "cloudflare:index/getZeroTrustInfrastructureAccessTargets:getZeroTrustInfrastructureAccessTargets".into(),
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "createdAfter".into(), value: args.created_after },
                ObjectField { name: "hostname".into(), value: args.hostname },
                ObjectField { name: "hostnameContains".into(), value: args.hostname_contains },
                ObjectField { name: "ipv4".into(), value: args.ipv4 },
                ObjectField { name: "ipv6".into(), value: args.ipv6 },
                ObjectField { name: "modifiedAfter".into(), value: args.modified_after },
                ObjectField { name: "virtualNetworkId".into(), value: args.virtual_network_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "createdAfter".into() },
                ResultField { name: "hostname".into() },
                ResultField { name: "hostnameContains".into() },
                ResultField { name: "id".into() },
                ResultField { name: "ipv4".into() },
                ResultField { name: "ipv6".into() },
                ResultField { name: "modifiedAfter".into() },
                ResultField { name: "targets".into() },
                ResultField { name: "virtualNetworkId".into() },
            ],
        };

        let o = invoke(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        get_zero_trust_infrastructure_access_targets::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created_after: hashmap.remove("createdAfter").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            hostname_contains: hashmap.remove("hostnameContains").unwrap(),
            id: hashmap.remove("id").unwrap(),
            ipv4: hashmap.remove("ipv4").unwrap(),
            ipv6: hashmap.remove("ipv6").unwrap(),
            modified_after: hashmap.remove("modifiedAfter").unwrap(),
            targets: hashmap.remove("targets").unwrap(),
            virtual_network_id: hashmap.remove("virtualNetworkId").unwrap(),
        }

    }
}
