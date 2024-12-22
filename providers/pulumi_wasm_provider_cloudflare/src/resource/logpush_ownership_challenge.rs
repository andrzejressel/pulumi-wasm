use std::collections::HashMap;
use crate::bindings::exports::pulumi::cloudflare::logpush_ownership_challenge;
use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use crate::Component;

impl logpush_ownership_challenge::Guest for Component {
    fn invoke(
        name: String,
        args: logpush_ownership_challenge::Args
    ) -> logpush_ownership_challenge::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/logpushOwnershipChallenge:LogpushOwnershipChallenge".into(),
            name,
            object: vec![
                ObjectField { name: "accountId".into(), value: args.account_id },
                ObjectField { name: "destinationConf".into(), value: args.destination_conf },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "accountId".into() },
                ResultField { name: "destinationConf".into() },
                ResultField { name: "ownershipChallengeFilename".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> = o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        logpush_ownership_challenge::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            destination_conf: hashmap.remove("destinationConf").unwrap(),
            ownership_challenge_filename: hashmap.remove("ownershipChallengeFilename").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }

    }
}
