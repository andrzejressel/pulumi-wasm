use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::teams_location;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl teams_location::Guest for Component {
    fn invoke(
        name: String,
        args: teams_location::Args
    ) -> teams_location::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsLocation:TeamsLocation".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "clientDefault".into(), value: args.client_default },
                ObjectField { name: "ecsSupport".into(), value: args.ecs_support },
                ObjectField { name: "name".into(), value: args.name },
                ObjectField { name: "networks".into(), value: args.networks },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "anonymizedLogsEnabled".into() },
                ResultField { name: "clientDefault".into() },
                ResultField { name: "dohSubdomain".into() },
                ResultField { name: "ecsSupport".into() },
                ResultField { name: "ip".into() },
                ResultField { name: "ipv4Destination".into() },
                ResultField { name: "name".into() },
                ResultField { name: "networks".into() },
                ResultField { name: "policyIds".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_location::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            anonymized_logs_enabled: hashmap.remove("anonymizedLogsEnabled").unwrap(),
            client_default: hashmap.remove("clientDefault").unwrap(),
            doh_subdomain: hashmap.remove("dohSubdomain").unwrap(),
            ecs_support: hashmap.remove("ecsSupport").unwrap(),
            ip: hashmap.remove("ip").unwrap(),
            ipv4_destination: hashmap.remove("ipv4Destination").unwrap(),
            name: hashmap.remove("name").unwrap(),
            networks: hashmap.remove("networks").unwrap(),
            policy_ids: hashmap.remove("policyIds").unwrap(),
        }

    }
}
